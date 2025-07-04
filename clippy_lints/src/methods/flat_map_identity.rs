use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::{MapFunc, is_trait_method};
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::{Span, sym};

use super::FLAT_MAP_IDENTITY;

/// lint use of `flat_map` for `Iterators` where `flatten` would be sufficient
pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx hir::Expr<'_>,
    flat_map_arg: &'tcx hir::Expr<'_>,
    flat_map_span: Span,
) {
    if is_trait_method(cx, expr, sym::Iterator) && MapFunc::from(flat_map_arg).is_expr_untyped_identity_function(cx) {
        span_lint_and_sugg(
            cx,
            FLAT_MAP_IDENTITY,
            flat_map_span.with_hi(expr.span.hi()),
            "use of `flat_map` with an identity function",
            "try",
            "flatten()".to_string(),
            Applicability::MachineApplicable,
        );
    }
}
