use clippy_config::Conf;
use clippy_config::types::InherentImplLintScope;
use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::fulfill_or_allowed;
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::def_id::LocalModDefId;
use rustc_hir::{Item, ItemKind, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::FileName;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for multiple inherent implementations of a struct
    ///
    /// The config option controls the scope in which multiple inherent `impl` blocks for the same
    /// struct are linted, allowing values of `module` (only within the same module), `file`
    /// (within the same file), or `crate` (anywhere in the crate, default).
    ///
    /// ### Why restrict this?
    /// Splitting the implementation of a type makes the code harder to navigate.
    ///
    /// ### Example
    /// ```no_run
    /// struct X;
    /// impl X {
    ///     fn one() {}
    /// }
    /// impl X {
    ///     fn other() {}
    /// }
    /// ```
    ///
    /// Could be written:
    ///
    /// ```no_run
    /// struct X;
    /// impl X {
    ///     fn one() {}
    ///     fn other() {}
    /// }
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub MULTIPLE_INHERENT_IMPL,
    restriction,
    "Multiple inherent impl that could be grouped"
}

impl_lint_pass!(MultipleInherentImpl => [MULTIPLE_INHERENT_IMPL]);

pub struct MultipleInherentImpl {
    scope: InherentImplLintScope,
}

impl MultipleInherentImpl {
    pub fn new(conf: &'static Conf) -> Self {
        Self {
            scope: conf.inherent_impl_lint_scope,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Criterion {
    Module(LocalModDefId),
    File(FileName),
    Crate,
}

impl<'tcx> LateLintPass<'tcx> for MultipleInherentImpl {
    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        let (impls, _) = cx.tcx.crate_inherent_impls(());
        for (&id, impl_ids) in &impls.inherent_impls {
            if impl_ids.len() < 2
            // Check for `#[expect]` or `#[allow]` on the type definition
            || fulfill_or_allowed(
                cx,
                MULTIPLE_INHERENT_IMPL,
                [cx.tcx.local_def_id_to_hir_id(id)],
            ) {
                continue;
            }

            let mut type_map = FxIndexMap::default();
            for impl_id in impl_ids.iter().map(|id| id.expect_local()) {
                let hir_id = cx.tcx.local_def_id_to_hir_id(impl_id);
                let span = if let Node::Item(&Item {
                    kind: ItemKind::Impl(impl_item),
                    span,
                    ..
                }) = cx.tcx.hir_node(hir_id)
                    && !span.from_expansion()
                    && impl_item.generics.params.is_empty()
                {
                    span
                } else {
                    continue;
                };
                let impl_ty = cx.tcx.type_of(impl_id).instantiate_identity();
                let criterion = match self.scope {
                    InherentImplLintScope::Module => Criterion::Module(cx.tcx.parent_module(hir_id)),
                    InherentImplLintScope::File => {
                        let span = cx.tcx.hir_span(hir_id);
                        Criterion::File(cx.tcx.sess.source_map().lookup_source_file(span.lo()).name.clone())
                    },
                    InherentImplLintScope::Crate => Criterion::Crate,
                };
                type_map
                    .entry((impl_ty, criterion))
                    .or_insert_with(Vec::new)
                    .push((span, hir_id));
            }

            for duplicates in type_map.into_values().filter(|v| v.len() >= 2) {
                // Only keep `impl` that don't `#[allow]` or `#[expect]`, and fulfill expectations
                let spans = duplicates
                    .into_iter()
                    .filter(|(_, hir_id)| !fulfill_or_allowed(cx, MULTIPLE_INHERENT_IMPL, [*hir_id]))
                    .map(|(span, _)| span)
                    .collect::<Vec<_>>();
                if spans.is_empty() {
                    continue;
                }
                // Choose the lowest file+pos as the reference `impl` to pinpoint in messages
                let source_map = cx.tcx.sess.source_map();
                let ref_span = spans
                    .iter()
                    .copied()
                    .min_by_key(|span| (source_map.lookup_char_pos(span.lo()).file.name.clone(), span.lo()))
                    .unwrap();
                // Emit lints
                for span in spans {
                    if span != ref_span {
                        span_lint_and_then(
                            cx,
                            MULTIPLE_INHERENT_IMPL,
                            span,
                            "multiple implementations of this structure",
                            |diag| {
                                diag.span_note(ref_span, "another implementation can be found here");
                            },
                        );
                    }
                }
            }
        }
    }
}
