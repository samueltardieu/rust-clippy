# Clippy Lint Renaming Table

This document proposes renaming Clippy lints to better align with the Rust RFC 0344 naming conventions ([link](https://rust-lang.github.io/rfcs/0344-conventions-galore.html)).

## Summary of Guidelines

The RFC 0344 guidelines for lint naming state:

1. **State the bad thing**: Lint names should state the bad thing being checked for (e.g., `deprecated`), so that `#[allow(deprecated)]` reads correctly.

2. **Avoid redundant suffixes**: Lints that apply to arbitrary items should just mention what they check for (e.g., use `deprecated` rather than `deprecated_items`). However, when removing a suffix would leave an adjective without a noun (e.g., `private_items` → `private`), the suffix should be retained for clarity.

3. **Use plural for specific classes**: If a lint applies to a specific grammatical class, mention that class and use the plural form (e.g., `unused_variables` rather than `unused_variable`). **This document takes an aggressive approach to pluralization**: all lints are pluralized unless they refer to concepts, patterns, or macros rather than countable instances.

4. **Prefer 'unused' terminology**: Lints that catch unnecessary, unused, or useless aspects of code should use the term `unused` (e.g., `unused_imports`, `unused_typecasts`).

5. **Use snake_case**: Use snake case consistently, as you would for function names.

## Pluralization Philosophy

This document applies pluralization comprehensively. The principle is:

- **Use plural** when a lint can find and report multiple instances in code (e.g., multiple borrows, multiple returns, multiple conversions)
- **Use singular** only for:
  - Macros (e.g., `dbg_macro` - refers to the macro itself)
  - Traits (e.g., `default_trait_access` - refers to the trait)
  - Patterns/concepts (e.g., `bool_comparison` - the comparison pattern)
  - Ordering/style concepts (e.g., `module_style`, `arbitrary_source_item_ordering`)
  - Mathematical concepts (e.g., `cognitive_complexity`)

## Proposed Renames

Out of 791 total Clippy lints analyzed, 45 lints were identified as not following RFC 0344 guidelines.

- **33 lints** use 'needless' instead of 'unnecessary' (all with pluralization)
- **9 lints** use 'useless' instead of 'unused' (all with pluralization)
- **2 lints** have redundant '_items' suffix (removed with pluralization)
- **Additional pluralization** applied to all renamed lints per guideline 3

| Current Name | Proposed Name | Reason |
|--------------|---------------|--------|
| `DOC_OVERINDENTED_LIST_ITEMS` | `DOC_OVERINDENTED_LISTS` | Has redundant suffix '_items' (guideline 2); should use plural |
| `NEEDLESS_ARBITRARY_SELF_TYPE` | `UNNECESSARY_ARBITRARY_SELF_TYPES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple type usages) |
| `NEEDLESS_AS_BYTES` | `UNNECESSARY_AS_BYTES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple conversions) |
| `NEEDLESS_BITWISE_BOOL` | `UNNECESSARY_BITWISE_BOOLS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple boolean expressions) |
| `NEEDLESS_BOOL` | `UNNECESSARY_BOOLS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple if-statements with booleans) |
| `NEEDLESS_BOOL_ASSIGN` | `UNNECESSARY_BOOL_ASSIGNS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple assignments) |
| `NEEDLESS_BORROW` | `UNNECESSARY_BORROWS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple borrow expressions) |
| `NEEDLESS_BORROWED_REFERENCE` | `UNNECESSARY_BORROWED_REFERENCES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple references) |
| `NEEDLESS_BORROWS_FOR_GENERIC_ARGS` | `UNNECESSARY_BORROWS_FOR_GENERIC_ARGS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_CHARACTER_ITERATION` | `UNNECESSARY_CHARACTER_ITERATIONS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple iterations) |
| `NEEDLESS_COLLECT` | `UNNECESSARY_COLLECTS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple collect calls) |
| `NEEDLESS_CONTINUE` | `UNNECESSARY_CONTINUES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple continue statements) |
| `NEEDLESS_DOCTEST_MAIN` | `UNNECESSARY_DOCTEST_MAINS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple doctests) |
| `NEEDLESS_ELSE` | `UNNECESSARY_ELSES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple else blocks) |
| `NEEDLESS_FOR_EACH` | `UNNECESSARY_FOR_EACHES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple for_each calls) |
| `NEEDLESS_IFS` | `UNNECESSARY_IFS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_LATE_INIT` | `UNNECESSARY_LATE_INITS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple initializations) |
| `NEEDLESS_LIFETIMES` | `UNNECESSARY_LIFETIMES` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_MATCH` | `UNNECESSARY_MATCHES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple match expressions) |
| `NEEDLESS_MAYBE_SIZED` | `UNNECESSARY_MAYBE_SIZEDS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple bounds) |
| `NEEDLESS_OPTION_AS_DEREF` | `UNNECESSARY_OPTION_AS_DEREFS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple method calls) |
| `NEEDLESS_OPTION_TAKE` | `UNNECESSARY_OPTION_TAKES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple take calls) |
| `NEEDLESS_PARENS_ON_RANGE_LITERALS` | `UNNECESSARY_PARENS_ON_RANGE_LITERALS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_PASS_BY_REF_MUT` | `UNNECESSARY_PASS_BY_REF_MUTS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple parameters) |
| `NEEDLESS_PASS_BY_VALUE` | `UNNECESSARY_PASS_BY_VALUES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple parameters) |
| `NEEDLESS_PUB_SELF` | `UNNECESSARY_PUB_SELFS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple visibility modifiers) |
| `NEEDLESS_QUESTION_MARK` | `UNNECESSARY_QUESTION_MARKS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple uses) |
| `NEEDLESS_RANGE_LOOP` | `UNNECESSARY_RANGE_LOOPS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple loops) |
| `NEEDLESS_RAW_STRINGS` | `UNNECESSARY_RAW_STRINGS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_RAW_STRING_HASHES` | `UNNECESSARY_RAW_STRING_HASHES` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_RETURN` | `UNNECESSARY_RETURNS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple return statements) |
| `NEEDLESS_RETURN_WITH_QUESTION_MARK` | `UNNECESSARY_RETURNS_WITH_QUESTION_MARK` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple returns) |
| `NEEDLESS_SPLITN` | `UNNECESSARY_SPLITNS` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple splitn calls) |
| `NEEDLESS_UPDATE` | `UNNECESSARY_UPDATES` | Uses 'needless' instead of 'unnecessary'; should use plural (checks multiple struct updates) |
| `NONMINIMAL_BOOL` | `NONMINIMAL_BOOLS` | Should use plural form (guideline 3) - checks multiple boolean expressions |
| `USED_UNDERSCORE_ITEMS` | `USED_UNDERSCORES` | Has redundant suffix '_items' (guideline 2); should use plural |
| `USELESS_ASREF` | `UNUSED_ASREFS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple as_ref calls) |
| `USELESS_ATTRIBUTE` | `UNUSED_ATTRIBUTES` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple attributes) |
| `USELESS_CONCAT` | `UNUSED_CONCATS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple concat calls) |
| `USELESS_CONVERSION` | `UNUSED_CONVERSIONS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple conversion calls) |
| `USELESS_FORMAT` | `UNUSED_FORMATS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple format calls) |
| `USELESS_LET_IF_SEQ` | `UNUSED_LET_IF_SEQS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple patterns) |
| `USELESS_NONZERO_NEW_UNCHECKED` | `UNUSED_NONZERO_NEW_UNCHECKEDS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple calls) |
| `USELESS_TRANSMUTE` | `UNUSED_TRANSMUTES` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple transmute calls) |
| `USELESS_VEC` | `UNUSED_VECS` | Uses 'useless' instead of 'unused' (guideline 4); should use plural (checks multiple vec constructions) |

## Lints Kept Singular (Examples)

The following categories of lints are appropriately kept singular:

### Macros
- `DBG_MACRO` - refers to the `dbg!` macro itself
- `TODO_MACRO` - refers to the `todo!` macro itself
- `PRINT_LITERAL` - refers to printing pattern

### Traits
- `DEFAULT_TRAIT_ACCESS` - refers to the `Default` trait
- `FUTURE_NOT_SEND` - refers to the `Send` trait bound

### Patterns and Concepts
- `BOOL_COMPARISON` - refers to the comparison pattern
- `MODULE_STYLE` - refers to module organization style
- `ARBITRARY_SOURCE_ITEM_ORDERING` - refers to ordering concept
- `COGNITIVE_COMPLEXITY` - refers to complexity metric

## Lints Requiring Further Review

The following lints may have names that don't clearly state the bad thing being checked for:

| Current Name | What it checks | Issue | Suggested Alternative |
|--------------|----------------|-------|----------------------|
| `ALLOW_ATTRIBUTES` | Usage of `#[allow]` instead of `#[expect]` | Name suggests allowing is bad, but the bad thing is missing expectations | Consider `MISSING_EXPECT_ATTRIBUTES` or `PREFER_EXPECT_OVER_ALLOW` |
| `ALLOW_ATTRIBUTES_WITHOUT_REASON` | `#[allow]` attributes without a reason parameter | Name is confusing with nested "allow" | Consider `MISSING_ALLOW_REASONS` or `UNDOCUMENTED_ALLOWS` |
| `USE_DEBUG` | Usage of Debug formatting in user-facing output | Name suggests positive action, bad thing is inappropriate Debug formatting | Consider `DEBUG_IN_USER_OUTPUTS` or `INAPPROPRIATE_DEBUG_FMTS` |
| `USE_SELF` | Unnecessary repetition of struct name instead of `Self` | Name suggests positive action, bad thing is unnecessary type repetition | Consider `UNNECESSARY_STRUCT_NAMES` or `MISSING_SELF_USAGES` |

**Note**: These suggestions use plural forms consistent with the pluralization philosophy.


## Rationale

### Why 'needless' should be 'unnecessary'

The RFC 0344 guidelines don't mention "needless" as a standard term. The term "unnecessary" is more standard in technical writing.

### Why 'useless' should be 'unused'

Per guideline 4, lints that catch "unnecessary, unused, or useless aspects of code should use the term `unused`".

### Why aggressive pluralization

Per guideline 3, when a lint checks multiple instances of a specific grammatical class, it should use the plural form. Since most lints can find multiple occurrences in code, plural forms are more accurate. For example:
- `UNNECESSARY_BORROWS` - a codebase can have many unnecessary borrows
- `UNNECESSARY_RETURNS` - a function can have multiple unnecessary return statements
- `UNUSED_CONVERSIONS` - code can contain multiple unused conversion calls

This makes `#[allow(unnecessary_borrows)]` read as "allow (multiple) unnecessary borrows" which is more accurate than the singular form.

## Examples

Before:
```rust
#[allow(needless_borrow)]  // Singular
let x = &value;

#[allow(needless_return)]  // Singular
fn foo() -> i32 { return 42; }

#[allow(useless_conversion)]  // Singular
let x: i32 = value.into();
```

After:
```rust
#[allow(unnecessary_borrows)]  // Plural - can have multiple
let x = &value;

#[allow(unnecessary_returns)]  // Plural - can have multiple
fn foo() -> i32 { return 42; }

#[allow(unused_conversions)]  // Plural - can have multiple
let x: i32 = value.into();
```

## Implementation Notes

- These are proposed renames only and require community discussion
- Actual implementation would require updating lint declarations, tests, and documentation
- Old lint names should be deprecated with clear migration paths
- Consider using lint group aliases for backward compatibility
- Update all documentation, including the Clippy book
