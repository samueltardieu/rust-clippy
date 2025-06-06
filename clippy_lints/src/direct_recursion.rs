use crate::rustc_lint::LintContext;
use clippy_config::Conf;
use clippy_utils::diagnostics::span_lint;
use clippy_utils::{get_attr, sym};
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Body, Expr, ExprKind, HirId, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Instance;
use rustc_session::impl_lint_pass;
use rustc_span::def_id::LocalDefId;
declare_clippy_lint! {
    /// ### What it does
    /// Checks for functions that call themselves from their body.
    ///
    /// ### Why restrict this?
    /// In Safety Critical contexts, recursive calls can lead to catastrophic
    /// crashes if they happen to overflow the stack.
    ///
    /// In most contexts, this is not an issue, so this lint is allow-by-default.
    ///
    /// ### Notes
    ///
    /// #### Triggers
    /// There are two things that trigger this lint:
    /// - Function calls from a function (or method) to itself,
    /// - Function pointer bindings from a function (or method) to itself.
    ///
    /// #### Independent of control flow
    /// This lint triggers whenever the conditions above are met, regardless of
    /// control flow and other such constructs.
    ///
    /// #### Blessing a recursive call
    /// The user may choose to bless a recursive call or binding using the
    /// attribute #[clippy::allowed_recursion]
    ///
    /// #### Indirect calls
    /// This lint **does not** detect indirect recursive calls.
    ///
    /// ### Examples
    /// This function will trigger the lint:
    /// ```no_run
    /// fn i_call_myself_in_a_bounded_way(bound: u8) {
    ///     if bound > 0 {
    ///         // This line will trigger the lint
    ///         i_call_myself_in_a_bounded_way(bound - 1);
    ///     }
    /// }
    /// ```
    /// Using #[clippy::allowed_recursion] lets it pass:
    /// ```no_run
    /// fn i_call_myself_in_a_bounded_way(bound: u8) {
    ///     if bound > 0 {
    ///         #[clippy::allowed_recursion]
    ///         i_call_myself_in_a_bounded_way(bound - 1);
    ///     }
    /// }
    /// ```
    /// This triggers the lint when `fibo` is bound to a function pointer
    /// inside `fibo`'s body
    /// ```no_run
    /// fn fibo(a: u32) -> u32 {
    ///     if a < 2 { a } else { (a - 2..a).map(fibo).sum() }
    /// }
    /// ```
    #[clippy::version = "1.89.0"]
    pub DIRECT_RECURSION,
    restriction,
    "functions shall not call themselves directly"
}

pub struct DirectRecursion {
    fn_id_stack: Vec<LocalDefId>,
    expr_check_blocker: Option<HirId>,
}

impl DirectRecursion {
    pub fn new(_: &'static Conf) -> Self {
        Self {
            // We preallocate a stack of size 4, because we'll probably need more than 2
            // but I really don't expect us to ever see more than 4 nested functions
            fn_id_stack: Vec::with_capacity(4),
            expr_check_blocker: None,
        }
    }

    /// Blocks checking more expressions, using `expr` as the key.
    fn block_with_expr(&mut self, expr: &Expr<'_>) {
        self.expr_check_blocker = Some(expr.hir_id);
    }

    /// Tells whether we're currently allowed to check expressions or not
    fn is_blocked(&self) -> bool {
        self.expr_check_blocker.is_some()
    }

    /// Tries opening the lock using `expr` as the key.
    fn try_unlocking_with(&mut self, expr: &Expr<'_>) {
        if let Some(key) = self.expr_check_blocker
            && key == expr.hir_id
        {
            self.expr_check_blocker = None;
        }
    }
}

impl_lint_pass!(DirectRecursion => [DIRECT_RECURSION]);

impl<'tcx> LateLintPass<'tcx> for DirectRecursion {
    /// Whenever we enter a Body, we push its owner's `DefId` into the stack
    fn check_body(&mut self, cx: &LateContext<'tcx>, body: &Body<'_>) {
        self.fn_id_stack.push(cx.tcx.hir_body_owner_def_id(body.id()));
    }

    /// We then revert this when we exit said `Body`
    fn check_body_post(&mut self, _: &LateContext<'tcx>, _: &Body<'_>) {
        _ = self.fn_id_stack.pop();
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        // We use this inner lock so that we avoid recursing if we've
        // already linted an expression that contains the expression
        // we're now visiting.
        // This lock is closed whenever we emit a lint, and it's opened
        // after we exit the node that closed it (see `check_expr_post`)
        if self.is_blocked() {
            return;
        }

        // Before running the lint, we look up the attributes of this Expr.
        // If it has been marked with `clippy::allowed_recursion`, then
        // we ignore it, as well as everything inside it; someone has
        // decided that the recursive calls within it are fine.
        let attrs = cx.tcx.hir_attrs(expr.hir_id);
        if get_attr(cx.sess(), attrs, sym::allowed_recursion).next().is_some() {
            self.block_with_expr(expr);
            return;
        }

        match expr.kind {
            ExprKind::MethodCall(_, _, _, _) => {
                // Uniquely identifying the `DefId` of method calls requires doing type checking.
                // `cx` takes care of that, and then we can ask for the `type_dependent_def_id`
                // of the `expr` whose kind is `ExprKind::MethodCall`.
                let typeck = cx.typeck_results();
                if let Some(type_resolved_def_id) = typeck.type_dependent_def_id(expr.hir_id) {
                    for stored_fn_id in &self.fn_id_stack {
                        if stored_fn_id.to_def_id() == type_resolved_def_id {
                            span_lint(cx, DIRECT_RECURSION, expr.span, "this method contains a call to itself");
                            self.block_with_expr(expr);
                            return;
                        }
                    }
                }
            },
            ExprKind::Call(path_expr, _) => {
                // This should almost always be true, as far as I'm aware.
                // `ExprKind::Call` values are supposed to contain an `Expr` of type `ExprKind::Path`
                // inside of them.
                if let ExprKind::Path(fn_qpath) = path_expr.kind {
                    match fn_qpath {
                        QPath::Resolved(_, path) => {
                            if let Res::Def(def_kind, def_id) = path.res {
                                for stored_fn_id in &self.fn_id_stack {
                                    if stored_fn_id.to_def_id() == def_id {
                                        let message = match def_kind {
                                            DefKind::Fn => "this function contains a call to itself",
                                            DefKind::AssocFn => "this associated function contains a call to itself",
                                            _ => unreachable!("this lint found something but it doesn't make sense"),
                                        };
                                        span_lint(cx, DIRECT_RECURSION, expr.span, message);
                                        self.block_with_expr(expr);
                                        return;
                                    }
                                }
                            }
                        },
                        QPath::TypeRelative(_, _) => {
                            // I'm still not sure this is proper.
                            // It definitely finds the right `DefId`, though.
                            let typeck = cx.typeck_results();
                            if let Some(id) = typeck.type_dependent_def_id(path_expr.hir_id)
                                && let args = typeck.node_args(path_expr.hir_id)
                                && let Ok(Some(fn_def)) = Instance::try_resolve(cx.tcx, cx.typing_env(), id, args)
                            {
                                let type_resolved_def_id = fn_def.def_id();

                                for stored_fn_id in &self.fn_id_stack {
                                    if stored_fn_id.to_def_id() == type_resolved_def_id {
                                        span_lint(
                                            cx,
                                            DIRECT_RECURSION,
                                            expr.span,
                                            "this function contains a call to itself",
                                        );
                                        self.block_with_expr(expr);
                                        return;
                                    }
                                }
                            }
                        },
                        QPath::LangItem(..) => {},
                    }
                }
            },
            // This branch takes care of finding bindings of function and method names
            // into fn pointers.
            ExprKind::Path(QPath::Resolved(_, path)) => {
                // Now we know that this Path is fully resolved.
                // We now must check if it points to a function or a method's definition.
                if let Res::Def(DefKind::Fn | DefKind::AssocFn, fn_path_id) = path.res {
                    // 1) Now we know that the path we've found is of a function or method definition.
                    //
                    // 2) We will now check if it corresponds to the path of a function we're inside
                    // of.
                    //
                    // 3) Thankfully, we've kept track of the functions that surround us, in
                    //`self.fn_id_stack`.
                    //
                    // 4) If the path that we've captured from `expr` coincides with one of the functions
                    // in the stack, then we know we have a recursive loop.

                    for fn_name in &self.fn_id_stack {
                        if fn_name.to_def_id() == fn_path_id {
                            span_lint(
                                cx,
                                DIRECT_RECURSION,
                                expr.span,
                                "this function contains a call to itself",
                            );
                            self.block_with_expr(expr);
                            return;
                        }
                    }
                }
            },
            _ => {},
        }
    }

    /// Every time we exit an `Expr` node, we see if we can unlock our Visitor
    /// using it as the key, just in case we blocked it after we entered it.
    fn check_expr_post(&mut self, _: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        self.try_unlocking_with(expr);
    }
}
