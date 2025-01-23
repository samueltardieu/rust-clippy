use cargo_metadata::{Edition, Metadata};
use clippy_utils::diagnostics::span_lint;
use rustc_lint::LateContext;
use rustc_span::DUMMY_SP;

use super::MISSING_MSRV;

pub(super) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    if let Some(package) = metadata.packages.first()
        && package.edition > Edition::E2021
        && package.rust_version.is_none()
        && !package.dependencies.is_empty()
        && package.publish.as_ref().is_none_or(|v| !v.is_empty())
    {
        span_lint(cx, MISSING_MSRV, DUMMY_SP, "missing `rust-version` field");
    }
}
