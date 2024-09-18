use clippy_utils::diagnostics::span_lint;
use rustc_hir::{ExprKind, Stmt, StmtKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _};
use rustc_middle::lint::in_external_macro;
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.83.0"]
    pub EMPTY_STATEMENT,
    style,
    "default lint description"
}

declare_lint_pass!(EmptyStatement => [EMPTY_STATEMENT]);

impl LateLintPass<'_> for EmptyStatement {
    fn check_stmt(&mut self, cx: &LateContext<'_>, stmt: &Stmt<'_>) {
        if in_external_macro(cx.sess(), stmt.span) {
            return;
        }
        if let StmtKind::Semi(expr) = stmt.kind
            && matches!(expr.kind, ExprKind::Block { .. })
            && !expr.span.from_expansion()
        {
            span_lint(cx, EMPTY_STATEMENT, expr.span, "consider removing the outer ';'");
        }
    }
}
