use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::match_def_path;
use clippy_utils::paths::SPAN;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty;
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `Span` manipulation that could be done more clearly
    /// using existing methods.
    ///
    pub MANUAL_SPAN_OPERATION,
    internal,
    "default lint description"
}

declare_lint_pass!(ManualSpanOperation => [MANUAL_SPAN_OPERATION]);

impl LateLintPass<'_> for ManualSpanOperation {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        if let ExprKind::MethodCall(outer_method, outer_receiver, [inner_expr], _) = expr.kind
            && let ExprKind::MethodCall(inner_method, inner_receiver, [], _) = inner_expr.kind
            && let ty::Adt(outer_receiver_adt, _) =
                cx.typeck_results().expr_ty_adjusted(outer_receiver).peel_refs().kind()
            && match_def_path(cx, outer_receiver_adt.did(), &SPAN)
            && let ty::Adt(inner_receiver_adt, _) = cx.typeck_results().expr_ty(inner_receiver).kind()
            && outer_receiver_adt.did() == inner_receiver_adt.did()
            && outer_method.ident.as_str() == "with_hi"
            && inner_method.ident.as_str() == "lo"
        {
            span_lint_and_then(
                cx,
                MANUAL_SPAN_OPERATION,
                expr.span,
                "manual implementation of `Span::until()`",
                |diag| {
                    diag.multipart_suggestion_verbose(
                        "use instead",
                        vec![
                            (outer_method.ident.span, "until".into()),
                            (inner_expr.span.with_lo(inner_receiver.span.hi()), String::new()),
                        ],
                        Applicability::MachineApplicable,
                    );
                },
            );
        }
    }
}
