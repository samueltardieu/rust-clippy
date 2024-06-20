use clippy_utils::diagnostics::span_lint_and_then;
use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Block, Expr, ExprKind, Item, ItemKind, Path, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::source_map::SourceMap;
use rustc_span::{Span, Symbol};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for expressions that use deferred constants declared in the same scope
    /// while a constant with the same name exists in an outer scope.
    ///
    /// ### Why is this bad?
    /// Lexically, it might look like the previously defined constant is used while,
    /// in reality, the one defined later in the current scope is used instead.
    ///
    /// ### Example
    /// The following code will print "world!world!".
    ///
    /// ```rust
    /// const MSG: &str = "Hello, ";
    /// {
    ///     print!("{}", MSG);
    ///     const MSG: &str = "world!";
    ///     println!("{}", MSG);
    /// }
    /// ```
    ///
    /// If this is what you indeed want, use instead:
    ///
    /// ```rust
    /// const MSG: &str = "Hello, ";
    /// {
    ///     const MSG: &str = "world!";
    ///     print!("{}", MSG);
    ///     println!("{}", MSG);
    /// }
    /// ```
    #[clippy::version = "1.81.0"]
    pub DEFERRED_CONST_SHADOW,
    suspicious,
    "using a deferred constant while a similar one exists in an outer scope"
}

#[derive(Default)]
pub(crate) struct DeferredConstShadow<'tcx> {
    consts: Vec<FxHashMap<Symbol, &'tcx Item<'tcx>>>,
}

impl_lint_pass!(DeferredConstShadow<'_> => [DEFERRED_CONST_SHADOW]);

impl<'tcx> LateLintPass<'tcx> for DeferredConstShadow<'tcx> {
    fn check_crate(&mut self, _: &LateContext<'tcx>) {
        assert!(self.consts.is_empty());
        self.consts.push(FxHashMap::default());
    }

    fn check_block(&mut self, _: &LateContext<'tcx>, _: &'tcx Block<'tcx>) {
        self.consts.push(FxHashMap::default());
    }

    fn check_block_post(&mut self, _: &LateContext<'tcx>, _: &'tcx Block<'tcx>) {
        self.consts.pop();
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Path(QPath::Resolved(
            _,
            Path {
                res: Res::Def(DefKind::Const, def_id),
                segments: [segment],
                span,
            },
        )) = expr.kind
            && !expr.span.from_expansion()
            && let Some(def_span) = cx.tcx.hir_span_if_local(*def_id)
            && span_in_same_file(cx.tcx.sess.source_map(), *span, def_span)
            && span < &def_span
        {
            for scope in self.consts.iter().rev().skip(1) {
                if let Some(shadowed) = scope.get(&segment.ident.name)
                    && shadowed.span != def_span
                {
                    span_lint_and_then(
                        cx,
                        DEFERRED_CONST_SHADOW,
                        expr.span,
                        "using a shadowing constant defined later in the same scope",
                        |diag| {
                            diag.span_note(
                                    shadowed.span,
                                    "this constant is shadowed in the current scope and will not be used",
                                )
                                .span_help(def_span, "this constant is used earlier in the scope and should be declared before it is referenced");
                        },
                    );
                    break;
                }
            }
        }
    }

    fn check_item(&mut self, _: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Const(ident, _, _, _) = item.kind
            && !item.span.from_expansion()
        {
            self.consts.last_mut().unwrap().insert(ident.name, item);
        }
    }
}

fn span_in_same_file(sm: &SourceMap, a: Span, b: Span) -> bool {
    sm.lookup_char_pos(a.lo()).file.stable_id == sm.lookup_char_pos(b.lo()).file.stable_id
}
