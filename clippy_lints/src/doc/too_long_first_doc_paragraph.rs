use rustc_errors::Applicability;
use rustc_hir::{Attribute, Item, ItemKind};
use rustc_lint::LateContext;

use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::is_from_proc_macro;
use clippy_utils::source::snippet_opt;

use super::TOO_LONG_FIRST_DOC_PARAGRAPH;

pub(super) fn check(
    cx: &LateContext<'_>,
    item: &Item<'_>,
    attrs: &[Attribute],
    mut first_paragraph_len: usize,
    check_private_items: bool,
) {
    if !check_private_items && !cx.effective_visibilities.is_exported(item.owner_id.def_id) {
        return;
    }
    if is_from_proc_macro(cx, item)
        || first_paragraph_len <= 200
        || !matches!(
            item.kind,
            // This is the list of items which can be documented AND are displayed on the module
            // page. So associated items or impl blocks are not part of this list.
            ItemKind::Static(..)
                | ItemKind::Const(..)
                | ItemKind::Fn { .. }
                | ItemKind::Macro(..)
                | ItemKind::Mod(..)
                | ItemKind::TyAlias(..)
                | ItemKind::Enum(..)
                | ItemKind::Struct(..)
                | ItemKind::Union(..)
                | ItemKind::Trait(..)
                | ItemKind::TraitAlias(..)
        )
    {
        return;
    }

    let mut spans = Vec::new();
    let mut should_suggest_empty_doc = false;

    for attr in attrs {
        if let Some(doc) = attr.doc_str() {
            spans.push(attr.span);
            let doc = doc.as_str().trim();
            if spans.len() == 1 {
                // We make this suggestion only if the first doc line ends with a punctuation
                // because it might just need to add an empty line with `///`.
                should_suggest_empty_doc = doc.ends_with(['.', '!', '?']);
            }
            let len = doc.chars().count();
            if len >= first_paragraph_len {
                break;
            }
            first_paragraph_len -= len;
        }
    }

    let &[first_span, .., last_span] = &spans[..] else {
        return;
    };

    span_lint_and_then(
        cx,
        TOO_LONG_FIRST_DOC_PARAGRAPH,
        first_span.until(last_span),
        "first doc comment paragraph is too long",
        |diag| {
            if should_suggest_empty_doc
                && let inter_span = first_span.between(spans[1])
                && let Some(new_line) = snippet_opt(cx, inter_span)
                && let Some(first_snippet) = snippet_opt(cx, first_span)
                && let Some(comment_prefix) = first_snippet.get(..3)
            {
                diag.span_suggestion(
                    inter_span,
                    "add an empty line",
                    format!("{new_line}{comment_prefix}{new_line}"),
                    Applicability::MachineApplicable,
                );
            }
        },
    );
}
