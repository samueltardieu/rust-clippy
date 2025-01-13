# Agent notes (rust-clippy)

## Build / lint / test
- Build: `cargo build` (workspace root).
- Lint: `cargo clippy` (use `--all-targets` if needed).
- Format: `cargo fmt` (rustfmt config in `rustfmt.toml`, max_width=120).
- Fast check: `cargo check`.
- UI tests: `cargo uitest`.
- Run a single UI test (or prefix): `TESTNAME=foo_functions cargo uitest`.
- Update UI test snapshots: `cargo bless` (or single: `TESTNAME=foo_functions cargo uibless`).
- Manual lint testing: `cargo dev lint <path/to/file/or/project>`.
- Run after adding a new lint: `cargo dev update`.
- Dogfood tests: `cargo dev dogfood`.
- Sanity checks: `cargo test -F internal`.

## Code style
- Prefer `cargo dev fmt` to reformat everything.
  - Run it before `cargo uibless`/`cargo bless` (if a test file is reformatted after blessing, the `.fixed` file will no longer be conformant).
  - Keep diffs minimal (especially from `cargo bless`).
- Imports: rustfmt enforces `imports_granularity = "Module"`; keep `use` lists tidy and sorted.
- Formatting: follow rustfmt 2024 style, wrap comments; avoid lines >120.
- Lint placement: add lints to the right module (e.g. `methods`, `operators`, etc.).
- Types: be explicit where clarity matters; lean on helper APIs in `clippy_utils`.
- Utilities: prefer using helpers from `clippy_utils` and add new ones there if they can be reused (check existing modules first).
- Naming: snake_case items/vars, CamelCase types, SCREAMING_SNAKE consts.
  - Lint names should follow what’s used in the Clippy book and match `clippy::lint_name`.
- Error handling/diagnostics: don’t call `LintContext::lint`/`span_lint` directly; use `clippy_utils::diagnostics::span_lint*` helpers.
  - If a computation is only needed when emitting the lint, do it inside `span_lint_and_then` / `span_lint_hir_and_then`.
- Suggestions:
  - Prefer multipart suggestions where applicable.
  - Prefer replacing parts of the existing code rather than reconstructing the full suggested code.
  - Use `Applicability` to reflect confidence (prefer `MachineApplicable` only when it’s clearly correct).
- Prefer readable matching (let-chains, helper fns) over deeply nested `match`/`if`.
- Tests:
  - Include cases where the lint should *not* trigger.
  - Include cases where relevant parts come from macros.
  - UI annotations: prefer matching on the lint name only (e.g. `//~^ borrow_deref_ref`), not free-form text like
    `ERROR: ...`.
  - UI tests don't need a `main`; if absent, the test file will be compiled as a library.
  - Some lints have dedicated test suites (`tests/ui-toml`, `tests/ui-cargo`) depending on what they lint/configure.
- Book/docs:
  - When adding new lint configuration options, run `cargo bless --test config-metadata` to update the book.
