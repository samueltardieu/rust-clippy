use rustc_ast::ByRef;
use rustc_errors::Applicability;
use rustc_hir::{Block, Body, Closure, Expr, ExprKind, HirId, Pat, PatKind, QPath, StmtKind, TyKind};
use rustc_lint::LateContext;
use rustc_span::Span;

use crate::source::snippet_with_applicability;
use crate::sugg::Sugg;
use crate::{path_to_local_id, sym};

/// Represents the argument of a `map`-like function, as a closure, as a path
/// in case η-reduction is used, or as an arbitrary expression.
#[derive(Debug)]
pub enum MapFunc<'hir> {
    Closure(&'hir Closure<'hir>, HirId, Span),
    Path(QPath<'hir>, HirId, Span),
    Expr(&'hir Expr<'hir>),
}

impl<'hir> From<&'hir Expr<'hir>> for MapFunc<'hir> {
    fn from(expr: &'hir Expr<'hir>) -> Self {
        match expr.kind {
            ExprKind::Closure(closure) => Self::Closure(closure, expr.hir_id, expr.span),
            ExprKind::Path(path) => Self::Path(path, expr.hir_id, expr.span),
            _ => Self::Expr(expr),
        }
    }
}

impl<'hir> MapFunc<'hir> {
    /// Checks if an expression represents the identity function
    /// Only examines closures and `std::convert::identity`
    ///
    /// NOTE: If you want to use this function to find out if a closure is unnecessary, you likely
    /// want to call [`is_expr_untyped_identity_function`] instead, which makes sure that the
    /// closure doesn't have type annotations. This is important because removing a closure with
    /// bindings can remove type information that helped type inference before, which can then
    /// lead to compile errors.
    pub fn is_expr_identity_function(&self, cx: &LateContext<'_>) -> bool {
        match self {
            Self::Closure(Closure { body, .. }, ..) => is_body_identity_function(cx, cx.tcx.hir_body(*body)),
            Self::Path(path, hir_id, _) => cx
                .qpath_res(path, *hir_id)
                .opt_def_id()
                .is_some_and(|id| cx.tcx.is_diagnostic_item(sym::convert_identity, id)),
            Self::Expr(_) => false,
        }
    }

    /// This is the same as [`MapFunc::is_expr_identity_function`], but does not consider closures
    /// with type annotations for its bindings (or similar) as identity functions:
    /// * `|x: u8| x`
    /// * `std::convert::identity::<u8>`
    pub fn is_expr_untyped_identity_function(&self, cx: &LateContext<'_>) -> bool {
        match self {
            Self::Closure(Closure { body, fn_decl, .. }, ..)
                if fn_decl.inputs.iter().all(|ty| matches!(ty.kind, TyKind::Infer(()))) =>
            {
                is_body_identity_function(cx, cx.tcx.hir_body(*body))
            },
            Self::Path(QPath::Resolved(_, path), ..)
                if path.segments.iter().all(|seg| seg.infer_args)
                    && let Some(did) = path.res.opt_def_id() =>
            {
                cx.tcx.is_diagnostic_item(sym::convert_identity, did)
            },
            _ => false,
        }
    }

    /// Build a suggestion suitable for use in a `.map()`-like function returning a boolean
    /// expression. η-expansion will be applied as needed. This will work only for a closure or
    /// a path, and will return `None` for an arbitrary expression.
    pub fn boolean_sugg(&self, cx: &LateContext<'hir>, invert: bool, app: &mut Applicability) -> Option<String> {
        match self {
            Self::Closure(closure, ..) => {
                let body = Sugg::hir_with_applicability(cx, cx.tcx.hir_body(closure.body).value, "..", app);
                Some(format!(
                    "{} {}",
                    snippet_with_applicability(cx, closure.fn_decl_span, "|..|", app),
                    if invert { !body } else { body }
                ))
            },
            Self::Path(_, _, span) => {
                let snippet = snippet_with_applicability(cx, *span, "_", app);
                if invert {
                    Some(format!("|x| !{snippet}(x)"))
                } else {
                    Some(snippet.to_string())
                }
            },
            Self::Expr(..) => None,
        }
    }
}

/// Checks if a function's body represents the identity function. Looks for bodies of the form:
/// * `|x| x`
/// * `|x| return x`
/// * `|x| { return x }`
/// * `|x| { return x; }`
/// * `|(x, y)| (x, y)`
///
/// Consider calling [`is_expr_untyped_identity_function`] or [`is_expr_identity_function`] instead.
fn is_body_identity_function(cx: &LateContext<'_>, func: &Body<'_>) -> bool {
    fn check_pat(cx: &LateContext<'_>, pat: &Pat<'_>, expr: &Expr<'_>) -> bool {
        if cx
            .typeck_results()
            .pat_binding_modes()
            .get(pat.hir_id)
            .is_some_and(|mode| matches!(mode.0, ByRef::Yes(_)))
        {
            // If a tuple `(x, y)` is of type `&(i32, i32)`, then due to match ergonomics,
            // the inner patterns become references. Don't consider this the identity function
            // as that changes types.
            return false;
        }

        match (pat.kind, expr.kind) {
            (PatKind::Binding(_, id, _, _), _) => {
                path_to_local_id(expr, id) && cx.typeck_results().expr_adjustments(expr).is_empty()
            },
            (PatKind::Tuple(pats, dotdot), ExprKind::Tup(tup))
                if dotdot.as_opt_usize().is_none() && pats.len() == tup.len() =>
            {
                pats.iter().zip(tup).all(|(pat, expr)| check_pat(cx, pat, expr))
            },
            _ => false,
        }
    }

    let [param] = func.params else {
        return false;
    };

    let mut expr = func.value;
    loop {
        match expr.kind {
            ExprKind::Block(
                &Block {
                    stmts: [],
                    expr: Some(e),
                    ..
                },
                _,
            )
            | ExprKind::Ret(Some(e)) => expr = e,
            ExprKind::Block(
                &Block {
                    stmts: [stmt],
                    expr: None,
                    ..
                },
                _,
            ) => {
                if let StmtKind::Semi(e) | StmtKind::Expr(e) = stmt.kind
                    && let ExprKind::Ret(Some(ret_val)) = e.kind
                {
                    expr = ret_val;
                } else {
                    return false;
                }
            },
            _ => return check_pat(cx, param.pat, expr),
        }
    }
}
