use clippy_config::Conf;
use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::msrvs::{self, Msrv};
use clippy_utils::source::{SpanRangeExt, position_before_rarrow};
use clippy_utils::{is_never_expr, is_unit_expr};
use rustc_errors::Applicability;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{
    AssocItemConstraintKind, Block, Body, Expr, ExprKind, FnDecl, FnRetTy, GenericArgsParentheses, Node, PolyTraitRef,
    Term, TyKind,
};
use rustc_hir_analysis::lower_ty;
use rustc_lexer::{TokenKind, tokenize};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::{BytePos, Span};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for unit (`()`) expressions that can be removed.
    ///
    /// ### Why is this bad?
    /// Such expressions add no value, but can make the code
    /// less readable. Depending on formatting they can make a `break` or `return`
    /// statement look like a function call.
    ///
    /// ### Example
    /// ```no_run
    /// fn return_unit() -> () {
    ///     ()
    /// }
    /// ```
    /// is equivalent to
    /// ```no_run
    /// fn return_unit() {}
    /// ```
    #[clippy::version = "1.31.0"]
    pub UNUSED_UNIT,
    style,
    "needless unit expression"
}

pub struct UnusedUnit {
    msrv: Msrv,
}

impl_lint_pass!(UnusedUnit => [UNUSED_UNIT]);

impl UnusedUnit {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv }
    }
}

impl<'tcx> LateLintPass<'tcx> for UnusedUnit {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        def_id: LocalDefId,
    ) {
        if let FnRetTy::Return(hir_ty) = decl.output
            && matches!(hir_ty.kind, TyKind::Tup([]))
            && !hir_ty.span.from_expansion()
            && get_def(span) == get_def(hir_ty.span)
        {
            // implicit types in closure signatures are forbidden when `for<...>` is present
            if let FnKind::Closure = kind
                && let Node::Expr(expr) = cx.tcx.hir_node_by_def_id(def_id)
                && let ExprKind::Closure(closure) = expr.kind
                && !closure.bound_generic_params.is_empty()
            {
                return;
            }

            // unit never type fallback is no longer supported since Rust 2024. For more information,
            // see <https://doc.rust-lang.org/nightly/edition-guide/rust-2024/never-type-fallback.html>
            if self.msrv.meets(cx, msrvs::UNIT_NEVER_TYPE_FALLBACK)
                && let ExprKind::Block(block, _) = body.value.kind
                && let Some(expr) = block.expr
                && is_never_expr(cx, expr).is_some()
            {
                return;
            }

            lint_unneeded_unit_return(cx, hir_ty.span, span);
        }
    }

    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        if let Some(expr) = block.expr
            && !expr.span.from_expansion()
            && is_block_expr_unit(cx, block, expr)
            && cx.tcx.hir_attrs(expr.hir_id).is_empty()
        // TODO: Handle macro passthrough
        {
            span_lint_and_sugg(
                cx,
                UNUSED_UNIT,
                expr.span,
                "unneeded unit expression",
                "remove the final `()`",
                String::new(),
                Applicability::MachineApplicable,
            );
        }
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Ret(Some(expr)) | ExprKind::Break(_, Some(expr)) = expr.kind
            && is_unit_expr(expr)
            && !expr.span.from_expansion()
        {
            span_lint_and_sugg(
                cx,
                UNUSED_UNIT,
                expr.span,
                "unneeded `()`",
                "remove the `()`",
                String::new(),
                Applicability::MachineApplicable,
            );
        }
    }

    fn check_poly_trait_ref(&mut self, cx: &LateContext<'tcx>, poly: &'tcx PolyTraitRef<'tcx>) {
        let segments = &poly.trait_ref.path.segments;

        if segments.len() == 1
            && ["Fn", "FnMut", "FnOnce"].contains(&segments[0].ident.name.as_str())
            && let Some(args) = segments[0].args
            && args.parenthesized == GenericArgsParentheses::ParenSugar
            && let constraints = &args.constraints
            && constraints.len() == 1
            && constraints[0].ident.name.as_str() == "Output"
            && let AssocItemConstraintKind::Equality { term: Term::Ty(hir_ty) } = constraints[0].kind
            && hir_ty.span != poly.span
            && !hir_ty.span.from_expansion()
            && let ty = lower_ty(cx.tcx, hir_ty)
            && ty.is_unit()
        {
            lint_unneeded_unit_return(cx, hir_ty.span, poly.span);
        }
    }
}

/// Check if the block expression is a unit expression by itself and is not the result of any
/// complex expression. Currently, it checks for:
/// - Empty block
/// - Unit expression with nested parens
/// - Unit expression from a passthrough macro
///
/// A passthrough macro refers to a macro that expands its input as-is. Currently it is very hard to
/// detect it because its expansion result is completely from user code, making its `SyntaxContext`
/// root. For more information, see https://github.com/rust-lang/rust-clippy/issues/4076
fn is_block_expr_unit(cx: &LateContext<'_>, block: &Block<'_>, expr: &Expr<'_>) -> bool {
    // Check for empty block
    if matches!(expr.kind, ExprKind::Tup([])) {
        // Check for unit expression with nested parens
        let Ok(snippet) = cx.tcx.sess.source_map().span_to_snippet(expr.span) else {
            return false;
        };
        let mut has_first_paren = false;
        for token in tokenize(&snippet) {
            match token.kind {
                TokenKind::OpenParen if has_first_paren => return false,
                TokenKind::OpenParen => has_first_paren = true,
                _ => {},
            }
        }

        // Check for passthrough macro by tokenizing the source before the unit expression
        let before = if let [.., stmt] = &block.stmts[..] {
            stmt.span.between(expr.span)
        } else {
            block.span.with_lo(BytePos(block.span.lo().0 + 1)).until(expr.span)
        };
        let Ok(snippet) = cx.tcx.sess.source_map().span_to_snippet(before) else {
            return false;
        };
        for token in tokenize(&snippet) {
            match token.kind {
                TokenKind::LineComment { .. }
                | TokenKind::BlockComment { .. }
                | TokenKind::Whitespace
                | TokenKind::InvalidIdent
                | TokenKind::UnknownPrefix
                | TokenKind::UnknownPrefixLifetime
                | TokenKind::Unknown
                | TokenKind::Eof => continue,
                _ => return false,
            }
        }

        return true;
    }

    false
}

// get the def site
#[must_use]
fn get_def(span: Span) -> Option<Span> {
    if span.from_expansion() {
        Some(span.ctxt().outer_expn_data().def_site)
    } else {
        None
    }
}

fn lint_unneeded_unit_return(cx: &LateContext<'_>, ty_span: Span, span: Span) {
    let (ret_span, appl) =
        span.with_hi(ty_span.hi())
            .get_source_text(cx)
            .map_or((ty_span, Applicability::MaybeIncorrect), |src| {
                position_before_rarrow(&src).map_or((ty_span, Applicability::MaybeIncorrect), |rpos| {
                    (
                        #[expect(clippy::cast_possible_truncation)]
                        ty_span.with_lo(BytePos(span.lo().0 + rpos as u32)),
                        Applicability::MachineApplicable,
                    )
                })
            });
    span_lint_and_sugg(
        cx,
        UNUSED_UNIT,
        ret_span,
        "unneeded unit return type",
        "remove the `-> ()`",
        String::new(),
        appl,
    );
}
