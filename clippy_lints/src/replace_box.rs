use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::res::{MaybeDef, MaybeResPath};
use clippy_utils::sugg::Sugg;
use clippy_utils::ty::implements_trait;
use clippy_utils::{is_default_equivalent_call, local_is_initialized};
use rustc_errors::Applicability;
use rustc_hir::{Body, Expr, ExprKind, HirId, HirIdSet, LangItem, Node, QPath};
use rustc_hir_typeck::expr_use_visitor::{Delegate, ExprUseVisitor, PlaceBase, PlaceWithHirId};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::FakeReadCause;
use rustc_middle::ty;
use rustc_session::impl_lint_pass;
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Detects assignments of `Default::default()` or `Box::new(value)`
    /// to a place of type `Box<T>`.
    ///
    /// ### Why is this bad?
    /// This incurs an extra heap allocation compared to assigning the boxed
    /// storage.
    ///
    /// ### Example
    /// ```no_run
    /// let mut b = Box::new(1u32);
    /// b = Default::default();
    /// ```
    /// Use instead:
    /// ```no_run
    /// let mut b = Box::new(1u32);
    /// *b = Default::default();
    /// ```
    #[clippy::version = "1.92.0"]
    pub REPLACE_BOX,
    perf,
    "assigning a newly created box to `Box<T>` is inefficient"
}

#[derive(Default)]
pub struct ReplaceBox {
    // Stack of caches for moved vars. The latest entry is the
    // body being currently visited.
    moved_vars_caches: Vec<Option<HirIdSet>>,
}

impl ReplaceBox {
    fn current_moved_vars_cache(&mut self) -> &mut Option<HirIdSet> {
        self.moved_vars_caches.last_mut().unwrap()
    }
}

impl_lint_pass!(ReplaceBox => [REPLACE_BOX]);

impl LateLintPass<'_> for ReplaceBox {
    fn check_body(&mut self, _: &LateContext<'_>, _: &Body<'_>) {
        self.moved_vars_caches.push(None);
    }

    fn check_body_post(&mut self, _: &LateContext<'_>, _: &Body<'_>) {
        _ = self.moved_vars_caches.pop();
    }

    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ Expr<'_>) {
        if let ExprKind::Assign(lhs, rhs, _) = &expr.kind
            && !lhs.span.from_expansion()
            && !rhs.span.from_expansion()
            && let lhs_ty = cx.typeck_results().expr_ty(lhs)
            // No diagnostic for late-initialized locals
            && let local = lhs.res_local_id()
            && local.is_none_or(|local| {
                local_is_initialized(cx, local)
                    && !self.current_moved_vars_cache().get_or_insert_with(|| {
                        let body_id = cx.enclosing_body.unwrap();
                        let mut ctx = MovedVariablesCtxt {
                            cx,
                            moved_vars: HirIdSet::default(),
                        };
                        ExprUseVisitor::for_clippy(cx, cx.tcx.hir_body_owner_def_id(body_id), &mut ctx)
                            .consume_body(cx.tcx.hir_body(body_id))
                            .into_ok();
                            ctx.moved_vars
                    }).contains(&local)
            })
            && let Some(inner_ty) = lhs_ty.boxed_ty()
        {
            if let Some(default_trait_id) = cx.tcx.get_diagnostic_item(sym::Default)
                && implements_trait(cx, inner_ty, default_trait_id, &[])
                && is_default_call(cx, rhs)
            {
                span_lint_and_then(
                    cx,
                    REPLACE_BOX,
                    expr.span,
                    "creating a new box with default content",
                    |diag| {
                        let mut app = Applicability::MachineApplicable;
                        let suggestion = format!(
                            "{} = Default::default()",
                            Sugg::hir_with_applicability(cx, lhs, "_", &mut app).deref()
                        );

                        diag.note("this creates a needless allocation").span_suggestion(
                            expr.span,
                            "replace existing content with default instead",
                            suggestion,
                            app,
                        );
                    },
                );
            }

            if inner_ty.is_sized(cx.tcx, cx.typing_env())
                && let Some(rhs_inner) = get_box_new_payload(cx, rhs)
            {
                span_lint_and_then(cx, REPLACE_BOX, expr.span, "creating a new box", |diag| {
                    let mut app = Applicability::MachineApplicable;
                    let suggestion = format!(
                        "{} = {}",
                        Sugg::hir_with_applicability(cx, lhs, "_", &mut app).deref(),
                        Sugg::hir_with_context(cx, rhs_inner, expr.span.ctxt(), "_", &mut app),
                    );

                    diag.note("this creates a needless allocation").span_suggestion(
                        expr.span,
                        "replace existing content with inner value instead",
                        suggestion,
                        app,
                    );
                });
            }
        }
    }
}

fn is_default_call(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    matches!(expr.kind, ExprKind::Call(func, _args) if is_default_equivalent_call(cx, func, Some(expr)))
}

fn get_box_new_payload<'tcx>(cx: &LateContext<'_>, expr: &Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    if let ExprKind::Call(box_new, [arg]) = expr.kind
        && let ExprKind::Path(QPath::TypeRelative(ty, seg)) = box_new.kind
        && seg.ident.name == sym::new
        && ty.basic_res().is_lang_item(cx, LangItem::OwnedBox)
    {
        Some(arg)
    } else {
        None
    }
}

struct MovedVariablesCtxt<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    moved_vars: HirIdSet,
}

impl<'tcx> Delegate<'tcx> for MovedVariablesCtxt<'_, 'tcx> {
    fn consume(&mut self, cmt: &PlaceWithHirId<'tcx>, _: HirId) {
        if let PlaceBase::Local(vid) = cmt.place.base
            && let Node::Expr(expr) = self.cx.tcx.hir_node(cmt.hir_id)
            && expr.res_local_id().is_some_and(|hir_id| hir_id == vid)
        {
            self.moved_vars.insert(vid);
        }
    }

    fn use_cloned(&mut self, _: &PlaceWithHirId<'tcx>, _: HirId) {}

    fn borrow(&mut self, _: &PlaceWithHirId<'tcx>, _: HirId, _: ty::BorrowKind) {}

    fn mutate(&mut self, _: &PlaceWithHirId<'tcx>, _: HirId) {}

    fn fake_read(&mut self, _: &PlaceWithHirId<'tcx>, _: FakeReadCause, _: HirId) {}
}
