# Clippy Lint Renaming Table

This document proposes renaming Clippy lints to better align with the Rust RFC 0344 naming conventions ([link](https://rust-lang.github.io/rfcs/0344-conventions-galore.html)).

## Summary of Guidelines

The RFC 0344 guidelines for lint naming state:

1. **State the bad thing**: Lint names should state the bad thing being checked for (e.g., `deprecated`), so that `#[allow(deprecated)]` reads correctly.

2. **Avoid redundant suffixes**: Lints that apply to arbitrary items should just mention what they check for (e.g., use `deprecated` rather than `deprecated_items`). However, when removing a suffix would leave an adjective without a noun (e.g., `private_items` → `private`), the suffix should be retained for clarity.

3. **Use plural for specific classes**: If a lint applies to a specific grammatical class, mention that class and use the plural form (e.g., `unused_variables` rather than `unused_variable`).

4. **Prefer 'unused' terminology**: Lints that catch unnecessary, unused, or useless aspects of code should use the term `unused` (e.g., `unused_imports`, `unused_typecasts`).

5. **Use snake_case**: Use snake case consistently, as you would for function names.

## Analysis Notes

### Plural Form Usage

The analysis respects that some lint names are appropriately singular when they:
- Refer to a concept or pattern rather than specific instances (e.g., `dbg_macro`)
- Apply to arbitrary items without specifying a grammatical class
- Are already plural in their base form (e.g., `blocks_in_conditions`)

Per guideline 3, pluralization is required when the lint applies to a specific grammatical class. For example:
- ✅ `unused_variables` - applies to the grammatical class of variables
- ✅ `dbg_macro` - refers to a macro pattern, not multiple macros
- ✅ `blocks_in_conditions` - already plural

### Suffix Removal Considerations

When considering removing suffixes like `_items`, it's important to ensure the resulting name is grammatically complete. Adjectives require nouns:
- ❌ `missing_docs_in_private` - 'private' is an adjective without a noun
- ✅ `missing_docs_in_private_items` - grammatically complete
- ✅ `doc_overindented_list` - 'list' is a noun, can remove '_items'

## Proposed Renames

Out of 791 total Clippy lints analyzed, 44 lints were identified as not following RFC 0344 guidelines:

- **33 lints** use 'needless' instead of 'unnecessary'
- **9 lints** use 'useless' instead of 'unused'
- **2 lints** have redundant '_items' suffix (where removal leaves a complete noun)

| Current Name | Proposed Name | Reason |
|--------------|---------------|--------|
| `DOC_OVERINDENTED_LIST_ITEMS` | `DOC_OVERINDENTED_LIST` | Has redundant suffix '_items' (guideline 2) |
| `NEEDLESS_ARBITRARY_SELF_TYPE` | `UNNECESSARY_ARBITRARY_SELF_TYPE` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_AS_BYTES` | `UNNECESSARY_AS_BYTES` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_BITWISE_BOOL` | `UNNECESSARY_BITWISE_BOOL` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_BOOL` | `UNNECESSARY_BOOL` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_BOOL_ASSIGN` | `UNNECESSARY_BOOL_ASSIGN` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_BORROW` | `UNNECESSARY_BORROW` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_BORROWED_REFERENCE` | `UNNECESSARY_BORROWED_REFERENCE` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_BORROWS_FOR_GENERIC_ARGS` | `UNNECESSARY_BORROWS_FOR_GENERIC_ARGS` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_CHARACTER_ITERATION` | `UNNECESSARY_CHARACTER_ITERATION` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_COLLECT` | `UNNECESSARY_COLLECT` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_CONTINUE` | `UNNECESSARY_CONTINUE` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_DOCTEST_MAIN` | `UNNECESSARY_DOCTEST_MAIN` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_ELSE` | `UNNECESSARY_ELSE` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_FOR_EACH` | `UNNECESSARY_FOR_EACH` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_IFS` | `UNNECESSARY_IFS` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_LATE_INIT` | `UNNECESSARY_LATE_INIT` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_LIFETIMES` | `UNNECESSARY_LIFETIMES` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_MATCH` | `UNNECESSARY_MATCH` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_MAYBE_SIZED` | `UNNECESSARY_MAYBE_SIZED` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_OPTION_AS_DEREF` | `UNNECESSARY_OPTION_AS_DEREF` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_OPTION_TAKE` | `UNNECESSARY_OPTION_TAKE` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_PARENS_ON_RANGE_LITERALS` | `UNNECESSARY_PARENS_ON_RANGE_LITERALS` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_PASS_BY_REF_MUT` | `UNNECESSARY_PASS_BY_REF_MUT` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_PASS_BY_VALUE` | `UNNECESSARY_PASS_BY_VALUE` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_PUB_SELF` | `UNNECESSARY_PUB_SELF` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_QUESTION_MARK` | `UNNECESSARY_QUESTION_MARK` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_RANGE_LOOP` | `UNNECESSARY_RANGE_LOOP` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_RAW_STRINGS` | `UNNECESSARY_RAW_STRINGS` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_RAW_STRING_HASHES` | `UNNECESSARY_RAW_STRING_HASHES` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_RETURN` | `UNNECESSARY_RETURN` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_RETURN_WITH_QUESTION_MARK` | `UNNECESSARY_RETURN_WITH_QUESTION_MARK` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_SPLITN` | `UNNECESSARY_SPLITN` | Uses 'needless' instead of 'unnecessary' |
| `NEEDLESS_UPDATE` | `UNNECESSARY_UPDATE` | Uses 'needless' instead of 'unnecessary' |
| `USED_UNDERSCORE_ITEMS` | `USED_UNDERSCORE` | Has redundant suffix '_items' (guideline 2) |
| `USELESS_ASREF` | `UNUSED_ASREF` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_ATTRIBUTE` | `UNUSED_ATTRIBUTE` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_CONCAT` | `UNUSED_CONCAT` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_CONVERSION` | `UNUSED_CONVERSION` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_FORMAT` | `UNUSED_FORMAT` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_LET_IF_SEQ` | `UNUSED_LET_IF_SEQ` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_NONZERO_NEW_UNCHECKED` | `UNUSED_NONZERO_NEW_UNCHECKED` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_TRANSMUTE` | `UNUSED_TRANSMUTE` | Uses 'useless' instead of 'unused' (guideline 4) |
| `USELESS_VEC` | `UNUSED_VEC` | Uses 'useless' instead of 'unused' (guideline 4) |

## Lints Requiring Further Review

The following lints may have names that don't clearly state the bad thing being checked for, or may be confusing:

| Current Name | What it checks | Issue | Suggested Alternative |
|--------------|----------------|-------|----------------------|
| `ALLOW_ATTRIBUTES` | Usage of `#[allow]` instead of `#[expect]` | Name suggests allowing is bad, but the bad thing is missing expectations | Consider `MISSING_EXPECT_ATTRIBUTE` or `PREFER_EXPECT_OVER_ALLOW` |
| `ALLOW_ATTRIBUTES_WITHOUT_REASON` | `#[allow]` attributes without a reason parameter | Name is confusing with nested "allow" | Consider `MISSING_ALLOW_REASON` or `UNDOCUMENTED_ALLOW` |
| `USE_DEBUG` | Usage of Debug formatting in user-facing output | Name suggests positive action, bad thing is inappropriate Debug formatting | Consider `DEBUG_IN_USER_OUTPUT` or `INAPPROPRIATE_DEBUG_FMT` |
| `USE_SELF` | Unnecessary repetition of struct name instead of `Self` | Name suggests positive action, bad thing is unnecessary type repetition | Consider `UNNECESSARY_STRUCT_NAME` or `MISSING_SELF_USAGE` |

**Note**: These suggestions aim to better align with guideline 1 (state the bad thing) while maintaining clarity about what the lint checks for. The current names either suggest positive actions or contain confusing terminology.


## Rationale

### Why 'needless' should be 'unnecessary'

The RFC 0344 guidelines don't mention "needless" as a standard term. The guidelines specify:
- Use "unused" for things that are not used (e.g., `unused_imports`, `unused_variables`)
- The term "unnecessary" is more standard in technical writing for things that serve no purpose

The distinction is subtle:
- "unused" = something that exists but is never used (e.g., an import that's never referenced)
- "unnecessary" = something that can be removed or simplified without changing behavior (e.g., an unnecessary borrow, an unnecessary return statement)

For example, `#[allow(unnecessary_borrow)]` reads better than `#[allow(needless_borrow)]` and is more precise: the borrow exists and may be used, but it's not necessary.

### Why 'useless' should be 'unused'

Per guideline 4, lints that catch "unnecessary, unused, or useless aspects of code should use the term `unused`". The term "useless" is colloquial and less precise than "unused".

### Why some '_items' suffixes should be removed

Per guideline 2, lints that apply to arbitrary items should just mention what they check for, keeping lint names short. For example, use `deprecated` rather than `deprecated_items`, as `#[allow(deprecated)]` items already reads correctly.

However, the suffix must be retained when removing it would leave an adjective without a noun, such as `missing_docs_in_private_items` where "private" is an adjective modifying "items". Removing "_items" would result in `missing_docs_in_private`, which is grammatically incomplete.

## Examples

Before:
```rust
#[allow(needless_borrow)]
fn foo(x: &str) { ... }

#[allow(useless_conversion)]
let x: i32 = x.into();

#[allow(doc_overindented_list_items)]
/// - item
fn documented() { ... }
```

After:
```rust
#[allow(unnecessary_borrow)]
fn foo(x: &str) { ... }

#[allow(unused_conversion)]
let x: i32 = x.into();

#[allow(doc_overindented_list)]  // 'list' is a noun
/// - item
fn documented() { ... }
```

But keep the suffix when needed:
```rust
// Keep: 'private' is an adjective that needs 'items'
#[allow(missing_docs_in_private_items)]
mod internal { }

// Keep: 'public' is an adjective that needs 'items'
#[allow(missing_inline_in_public_items)]
pub fn api() { }
```

## Implementation Notes

- These are proposed renames only and require community discussion
- Actual implementation would require updating lint declarations, tests, and documentation
- Old lint names should be deprecated with clear migration paths to avoid breaking existing code
- Consider using lint group aliases to maintain backward compatibility during transition period
- Update all documentation, including the Clippy book and online lint list
- Ensure rustfix suggestions work correctly with renamed lints
