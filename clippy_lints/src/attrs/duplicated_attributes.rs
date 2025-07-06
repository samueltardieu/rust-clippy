use super::DUPLICATED_ATTRIBUTES;
use clippy_utils::diagnostics::span_lint_and_then;
use rustc_ast::{Attribute, MetaItem};
use rustc_data_structures::fx::FxHashMap;
use rustc_lint::EarlyContext;
use rustc_span::{Span, Symbol, sym};
use std::collections::hash_map::Entry;

type AttrValue = (Vec<Symbol>, Option<Symbol>);
type DuplicatedMap = FxHashMap<AttrValue, Span>;

fn emit_if_duplicated(
    cx: &EarlyContext<'_>,
    attr: &MetaItem,
    attr_paths: &mut DuplicatedMap,
    attr_value: AttrValue,
) {
    match attr_paths.entry(attr_value) {
        Entry::Vacant(v) => {
            v.insert(attr.span);
        },
        Entry::Occupied(o) => {
            span_lint_and_then(cx, DUPLICATED_ATTRIBUTES, attr.span, "duplicated attribute", |diag| {
                diag.span_note(*o.get(), "first defined here");
                diag.span_help(attr.span, "remove this attribute");
            });
        },
    }
}

fn check_duplicated_attr(
    cx: &EarlyContext<'_>,
    attr: &MetaItem,
    attr_paths: &mut DuplicatedMap,
    parent: &mut Vec<Symbol>,
) {
    if attr.span.from_expansion() {
        return;
    }
    if let Some(ident) = attr.ident() {
        let name = ident.name;
        if name == sym::doc || name == sym::cfg_attr_trace || name == sym::rustc_on_unimplemented || name == sym::reason
        {
            // FIXME: Would be nice to handle `cfg_attr` as well. Only problem is to check that cfg
            // conditions are the same.
            // `#[rustc_on_unimplemented]` contains duplicated subattributes, that's expected.
            return;
        }
        if let Some(direct_parent) = parent.last()
            && *direct_parent == sym::cfg_trace
            && [sym::all, sym::not, sym::any].contains(&name)
        {
            // FIXME: We don't correctly check `cfg`s for now, so if it's more complex than just a one
            // level `cfg`, we leave.
            return;
        }
    }
    let old_len = parent.len();
    for segment in &attr.path.segments {
        parent.push(segment.ident.name);
    }
    if let Some(value) = attr.value_str() {
        emit_if_duplicated(
            cx,
            attr,
            attr_paths,
            (parent.clone(), Some(value)),
        );
    } else if let Some(sub_attrs) = attr.meta_item_list() {
        for sub_attr in sub_attrs {
            if let Some(meta) = sub_attr.meta_item() {
                check_duplicated_attr(cx, meta, attr_paths, parent);
            }
        }
    } else {
        emit_if_duplicated(cx, attr, attr_paths, (parent.clone(), None));
    }
    parent.truncate(old_len);
}

pub fn check(cx: &EarlyContext<'_>, attrs: &[Attribute]) {
    let mut attr_paths = FxHashMap::default();
    let mut current_attr = Vec::new();

    for attr in attrs {
        if let Some(meta) = attr.meta() {
            check_duplicated_attr(cx, &meta, &mut attr_paths, &mut current_attr);
        }
    }
}
