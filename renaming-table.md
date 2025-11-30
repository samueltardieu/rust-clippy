# Clippy Lint Renaming Table

This document proposes renaming Clippy lints to better align with the Rust RFC 0344 naming conventions ([link](https://rust-lang.github.io/rfcs/0344-conventions-galore.html)).

## Summary of Guidelines

The RFC 0344 guidelines for lint naming state:

1. **State the bad thing**: Lint names should state the bad thing being checked for (e.g., `deprecated`), so that `#[allow(deprecated)]` reads correctly.

2. **Avoid redundant suffixes**: Lints that apply to arbitrary items should just mention what they check for (e.g., use `deprecated` rather than `deprecated_items`). However, when removing a suffix would leave an adjective without a noun (e.g., `private_items` → `private`), the suffix should be retained for clarity.

3. **Use plural for specific classes**: If a lint applies to a specific grammatical class, mention that class and use the plural form (e.g., `unused_variables` rather than `unused_variable`). This document applies pluralization to all lint names ending in countable nouns, except those with convincing arguments to stay singular.

4. **Prefer 'unused' terminology**: Lints that catch unnecessary, unused, or useless aspects of code should use the term `unused` (e.g., `unused_imports`, `unused_typecasts`).

5. **Use snake_case**: Use snake case consistently, as you would for function names.

## Pluralization Philosophy

This document applies pluralization comprehensively to all lint names ending in countable nouns. The principle is:

- **Use plural** when the last word is a countable noun and the lint can find multiple instances
- **Use singular** only for:
  - Macros (e.g., `dbg_macro` - refers to the macro itself)
  - Traits (e.g., `default_trait_access` - refers to the trait concept)
  - Patterns/concepts (e.g., `assign_op_pattern` - the pattern itself)
  - Style/ordering/convention concepts
  - Mathematical concepts (e.g., `cognitive_complexity`)
  - Configuration concepts (e.g., `cargo_common_metadata`)

**Note**: Pluralization is only applied when grammatically correct - the last significant word must be a noun, not a verb or adjective.

## Proposed Renames

Out of 791 total Clippy lints analyzed, 199 lints were identified as not following RFC 0344 guidelines.

- **33 lints** use 'needless' instead of 'unnecessary' (all with plural forms)
- **9 lints** use 'useless' instead of 'unused' (all with plural forms)
- **2 lints** have redundant '_items' suffix (removed with pluralization)
- **154 additional lints** need pluralization (noun endings)

| Current Name | Proposed Name | Reason |
|--------------|---------------|--------|
| `ALMOST_COMPLETE_RANGE` | `ALMOST_COMPLETE_RANGES` | Should use plural form (guideline 3) - 'range' is a countable noun |
| `APPROX_CONSTANT` | `APPROX_CONSTANTS` | Should use plural form (guideline 3) - 'constant' is a countable noun |
| `AWAIT_HOLDING_INVALID_TYPE` | `AWAIT_HOLDING_INVALID_TYPES` | Should use plural form (guideline 3) - 'type' is a countable noun |
| `AWAIT_HOLDING_LOCK` | `AWAIT_HOLDING_LOCKS` | Should use plural form (guideline 3) - 'lock' is a countable noun |
| `AWAIT_HOLDING_REFCELL_REF` | `AWAIT_HOLDING_REFCELL_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `BAD_BIT_MASK` | `BAD_BIT_MASKS` | Should use plural form (guideline 3) - 'mask' is a countable noun |
| `BOOL_ASSERT_COMPARISON` | `BOOL_ASSERT_COMPARISONS` | Should use plural form (guideline 3) - 'comparison' is a countable noun |
| `BOOL_COMPARISON` | `BOOL_COMPARISONS` | Should use plural form (guideline 3) - 'comparison' is a countable noun |
| `BORROWED_BOX` | `BORROWED_BOXES` | Should use plural form (guideline 3) - 'box' is a countable noun |
| `BORROW_AS_PTR` | `BORROW_AS_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `BORROW_DEREF_REF` | `BORROW_DEREF_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `BORROW_INTERIOR_MUTABLE_CONST` | `BORROW_INTERIOR_MUTABLE_CONSTS` | Should use plural form (guideline 3) - 'const' is a countable noun |
| `BOXED_LOCAL` | `BOXED_LOCALS` | Should use plural form (guideline 3) - 'local' is a countable noun |
| `BOX_COLLECTION` | `BOX_COLLECTIONS` | Should use plural form (guideline 3) - 'collection' is a countable noun |
| `CAST_ENUM_CONSTRUCTOR` | `CAST_ENUM_CONSTRUCTORS` | Should use plural form (guideline 3) - 'constructor' is a countable noun |
| `CAST_ENUM_TRUNCATION` | `CAST_ENUM_TRUNCATIONS` | Should use plural form (guideline 3) - 'truncation' is a countable noun |
| `CAST_POSSIBLE_TRUNCATION` | `CAST_POSSIBLE_TRUNCATIONS` | Should use plural form (guideline 3) - 'truncation' is a countable noun |
| `CAST_PRECISION_LOSS` | `CAST_PRECISION_LOSSES` | Should use plural form (guideline 3) - 'loss' is a countable noun |
| `CAST_PTR_ALIGNMENT` | `CAST_PTR_ALIGNMENTS` | Should use plural form (guideline 3) - 'alignment' is a countable noun |
| `CAST_SIGN_LOSS` | `CAST_SIGN_LOSSES` | Should use plural form (guideline 3) - 'loss' is a countable noun |
| `CLONE_ON_REF_PTR` | `CLONE_ON_REF_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `COLLAPSIBLE_MATCH` | `COLLAPSIBLE_MATCHES` | Should use plural form (guideline 3) - 'match' is a countable noun |
| `CONFUSING_METHOD_TO_NUMERIC_CAST` | `CONFUSING_METHOD_TO_NUMERIC_CASTS` | Should use plural form (guideline 3) - 'cast' is a countable noun |
| `COPY_ITERATOR` | `COPY_ITERATORS` | Should use plural form (guideline 3) - 'iterator' is a countable noun |
| `CREATE_DIR` | `CREATE_DIRS` | Should use plural form (guideline 3) - 'dir' is a countable noun |
| `CROSSPOINTER_TRANSMUTE` | `CROSSPOINTER_TRANSMUTES` | Should use plural form (guideline 3) - 'transmute' is a countable noun |
| `DEBUG_ASSERT_WITH_MUT_CALL` | `DEBUG_ASSERT_WITH_MUT_CALLS` | Should use plural form (guideline 3) - 'call' is a countable noun |
| `DECIMAL_LITERAL_REPRESENTATION` | `DECIMAL_LITERAL_REPRESENTATIONS` | Should use plural form (guideline 3) - 'representation' is a countable noun |
| `DECLARE_INTERIOR_MUTABLE_CONST` | `DECLARE_INTERIOR_MUTABLE_CONSTS` | Should use plural form (guideline 3) - 'const' is a countable noun |
| `DEFAULT_NUMERIC_FALLBACK` | `DEFAULT_NUMERIC_FALLBACKS` | Should use plural form (guideline 3) - 'fallback' is a countable noun |
| `DEFAULT_UNION_REPRESENTATION` | `DEFAULT_UNION_REPRESENTATIONS` | Should use plural form (guideline 3) - 'representation' is a countable noun |
| `DEPRECATED_SEMVER` | `DEPRECATED_SEMVERS` | Should use plural form (guideline 3) - 'semver' is a countable noun |
| `DIVERGING_SUB_EXPRESSION` | `DIVERGING_SUB_EXPRESSIONS` | Should use plural form (guideline 3) - 'expression' is a countable noun |
| `DOC_BROKEN_LINK` | `DOC_BROKEN_LINKS` | Should use plural form (guideline 3) - 'link' is a countable noun |
| `DOC_LAZY_CONTINUATION` | `DOC_LAZY_CONTINUATIONS` | Should use plural form (guideline 3) - 'continuation' is a countable noun |
| `DOC_OVERINDENTED_LIST_ITEMS` | `DOC_OVERINDENTED_LISTS` | Has redundant '_items' suffix (guideline 2); plural form |
| `DOC_PARAGRAPHS_MISSING_PUNCTUATION` | `DOC_PARAGRAPHS_MISSING_PUNCTUATIONS` | Should use plural form (guideline 3) - 'punctuation' is a countable noun |
| `DUPLICATE_MOD` | `DUPLICATE_MODS` | Should use plural form (guideline 3) - 'mod' is a countable noun |
| `DUPLICATE_UNDERSCORE_ARGUMENT` | `DUPLICATE_UNDERSCORE_ARGUMENTS` | Should use plural form (guideline 3) - 'argument' is a countable noun |
| `EAGER_TRANSMUTE` | `EAGER_TRANSMUTES` | Should use plural form (guideline 3) - 'transmute' is a countable noun |
| `EMPTY_LOOP` | `EMPTY_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `ENUM_CLIKE_UNPORTABLE_VARIANT` | `ENUM_CLIKE_UNPORTABLE_VARIANTS` | Should use plural form (guideline 3) - 'variant' is a countable noun |
| `ERROR_IMPL_ERROR` | `ERROR_IMPL_ERRORS` | Should use plural form (guideline 3) - 'error' is a countable noun |
| `EXCESSIVE_NESTING` | `EXCESSIVE_NESTINGS` | Should use plural form (guideline 3) - 'nesting' is a countable noun |
| `EXCESSIVE_PRECISION` | `EXCESSIVE_PRECISIONS` | Should use plural form (guideline 3) - 'precision' is a countable noun |
| `EXPECT_FUN_CALL` | `EXPECT_FUN_CALLS` | Should use plural form (guideline 3) - 'call' is a countable noun |
| `EXPLICIT_COUNTER_LOOP` | `EXPLICIT_COUNTER_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `EXPLICIT_INTO_ITER_LOOP` | `EXPLICIT_INTO_ITER_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `EXPLICIT_ITER_LOOP` | `EXPLICIT_ITER_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `FLAT_MAP_OPTION` | `FLAT_MAP_OPTIONS` | Should use plural form (guideline 3) - 'option' is a countable noun |
| `FLOAT_CMP_CONST` | `FLOAT_CMP_CONSTS` | Should use plural form (guideline 3) - 'const' is a countable noun |
| `FN_TO_NUMERIC_CAST` | `FN_TO_NUMERIC_CASTS` | Should use plural form (guideline 3) - 'cast' is a countable noun |
| `FN_TO_NUMERIC_CAST_WITH_TRUNCATION` | `FN_TO_NUMERIC_CAST_WITH_TRUNCATIONS` | Should use plural form (guideline 3) - 'truncation' is a countable noun |
| `FORMAT_PUSH_STRING` | `FORMAT_PUSH_STRINGS` | Should use plural form (guideline 3) - 'string' is a countable noun |
| `FROM_RAW_WITH_VOID_PTR` | `FROM_RAW_WITH_VOID_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `IMPLICIT_HASHER` | `IMPLICIT_HASHERS` | Should use plural form (guideline 3) - 'hasher' is a countable noun |
| `INCONSISTENT_STRUCT_CONSTRUCTOR` | `INCONSISTENT_STRUCT_CONSTRUCTORS` | Should use plural form (guideline 3) - 'constructor' is a countable noun |
| `INDEX_REFUTABLE_SLICE` | `INDEX_REFUTABLE_SLICES` | Should use plural form (guideline 3) - 'slice' is a countable noun |
| `INEFFECTIVE_BIT_MASK` | `INEFFECTIVE_BIT_MASKS` | Should use plural form (guideline 3) - 'mask' is a countable noun |
| `INEFFICIENT_TO_STRING` | `INEFFICIENT_TO_STRINGS` | Should use plural form (guideline 3) - 'string' is a countable noun |
| `INFALLIBLE_DESTRUCTURING_MATCH` | `INFALLIBLE_DESTRUCTURING_MATCHES` | Should use plural form (guideline 3) - 'match' is a countable noun |
| `INFINITE_ITER` | `INFINITE_ITERS` | Should use plural form (guideline 3) - 'iter' is a countable noun |
| `INFINITE_LOOP` | `INFINITE_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `INHERENT_TO_STRING` | `INHERENT_TO_STRINGS` | Should use plural form (guideline 3) - 'string' is a countable noun |
| `INTEGER_DIVISION` | `INTEGER_DIVISIONS` | Should use plural form (guideline 3) - 'division' is a countable noun |
| `INTO_ITER_ON_REF` | `INTO_ITER_ON_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `INTO_ITER_WITHOUT_ITER` | `INTO_ITER_WITHOUT_ITERS` | Should use plural form (guideline 3) - 'iter' is a countable noun |
| `IO_OTHER_ERROR` | `IO_OTHER_ERRORS` | Should use plural form (guideline 3) - 'error' is a countable noun |
| `IP_CONSTANT` | `IP_CONSTANTS` | Should use plural form (guideline 3) - 'constant' is a countable noun |
| `ITEMS_AFTER_TEST_MODULE` | `ITEMS_AFTER_TEST_MODULES` | Should use plural form (guideline 3) - 'module' is a countable noun |
| `ITER_FILTER_IS_OK` | `ITER_FILTERS_IS_OK` | Should use plural form (guideline 3) - 'filter' is a countable noun |
| `ITER_NEXT_LOOP` | `ITER_NEXT_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `ITER_NEXT_SLICE` | `ITER_NEXT_SLICES` | Should use plural form (guideline 3) - 'slice' is a countable noun |
| `ITER_NOT_RETURNING_ITERATOR` | `ITER_NOT_RETURNING_ITERATORS` | Should use plural form (guideline 3) - 'iterator' is a countable noun |
| `ITER_OVER_HASH_TYPE` | `ITER_OVER_HASH_TYPES` | Should use plural form (guideline 3) - 'type' is a countable noun |
| `ITER_WITHOUT_INTO_ITER` | `ITER_WITHOUT_INTO_ITERS` | Should use plural form (guideline 3) - 'iter' is a countable noun |
| `LARGE_ENUM_VARIANT` | `LARGE_ENUM_VARIANTS` | Should use plural form (guideline 3) - 'variant' is a countable noun |
| `LET_UNDERSCORE_LOCK` | `LET_UNDERSCORE_LOCKS` | Should use plural form (guideline 3) - 'lock' is a countable noun |
| `LOSSY_FLOAT_LITERAL` | `LOSSY_FLOAT_LITERALS` | Should use plural form (guideline 3) - 'literal' is a countable noun |
| `MANUAL_DANGLING_PTR` | `MANUAL_DANGLING_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `MANUAL_FILTER` | `MANUAL_FILTERS` | Should use plural form (guideline 3) - 'filter' is a countable noun |
| `MANUAL_OPTION_AS_SLICE` | `MANUAL_OPTION_AS_SLICES` | Should use plural form (guideline 3) - 'slice' is a countable noun |
| `MANUAL_SLICE_SIZE_CALCULATION` | `MANUAL_SLICE_SIZE_CALCULATIONS` | Should use plural form (guideline 3) - 'calculation' is a countable noun |
| `MATCH_AS_REF` | `MATCH_AS_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `MAYBE_INFINITE_ITER` | `MAYBE_INFINITE_ITERS` | Should use plural form (guideline 3) - 'iter' is a countable noun |
| `MISMATCHING_TYPE_PARAM_ORDER` | `MISMATCHING_TYPE_PARAM_ORDERS` | Should use plural form (guideline 3) - 'order' is a countable noun |
| `MISSING_CONST_FOR_FN` | `MISSING_CONSTS_FOR_FN` | Should use plural form (guideline 3) - 'const' is a countable noun |
| `MISSING_CONST_FOR_THREAD_LOCAL` | `MISSING_CONST_FOR_THREAD_LOCALS` | Should use plural form (guideline 3) - 'local' is a countable noun |
| `MISSING_SPIN_LOOP` | `MISSING_SPIN_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `MIXED_READ_WRITE_IN_EXPRESSION` | `MIXED_READ_WRITE_IN_EXPRESSIONS` | Should use plural form (guideline 3) - 'expression' is a countable noun |
| `MODULE_INCEPTION` | `MODULE_INCEPTIONS` | Should use plural form (guideline 3) - 'inception' is a countable noun |
| `MUTABLE_KEY_TYPE` | `MUTABLE_KEY_TYPES` | Should use plural form (guideline 3) - 'type' is a countable noun |
| `MUTEX_INTEGER` | `MUTEX_INTEGERS` | Should use plural form (guideline 3) - 'integer' is a countable noun |
| `MUT_FROM_REF` | `MUT_FROM_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `MUT_MUTEX_LOCK` | `MUT_MUTEX_LOCKS` | Should use plural form (guideline 3) - 'lock' is a countable noun |
| `NEEDLESS_ARBITRARY_SELF_TYPE` | `UNNECESSARY_ARBITRARY_SELF_TYPES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple type usages) |
| `NEEDLESS_AS_BYTES` | `UNNECESSARY_AS_BYTES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple conversions) |
| `NEEDLESS_BITWISE_BOOL` | `UNNECESSARY_BITWISE_BOOLS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple expressions) |
| `NEEDLESS_BOOL` | `UNNECESSARY_BOOLS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple if-statements) |
| `NEEDLESS_BOOL_ASSIGN` | `UNNECESSARY_BOOL_ASSIGNS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple assignments) |
| `NEEDLESS_BORROW` | `UNNECESSARY_BORROWS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple borrows) |
| `NEEDLESS_BORROWED_REFERENCE` | `UNNECESSARY_BORROWED_REFERENCES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple references) |
| `NEEDLESS_BORROWS_FOR_GENERIC_ARGS` | `UNNECESSARY_BORROWS_FOR_GENERIC_ARGS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_CHARACTER_ITERATION` | `UNNECESSARY_CHARACTER_ITERATIONS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple iterations) |
| `NEEDLESS_COLLECT` | `UNNECESSARY_COLLECTS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple calls) |
| `NEEDLESS_CONTINUE` | `UNNECESSARY_CONTINUES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple statements) |
| `NEEDLESS_DOCTEST_MAIN` | `UNNECESSARY_DOCTEST_MAINS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple doctests) |
| `NEEDLESS_ELSE` | `UNNECESSARY_ELSES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple blocks) |
| `NEEDLESS_FOR_EACH` | `UNNECESSARY_FOR_EACHES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple calls) |
| `NEEDLESS_IFS` | `UNNECESSARY_IFS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_LATE_INIT` | `UNNECESSARY_LATE_INITS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple initializations) |
| `NEEDLESS_LIFETIMES` | `UNNECESSARY_LIFETIMES` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_MATCH` | `UNNECESSARY_MATCHES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple expressions) |
| `NEEDLESS_MAYBE_SIZED` | `UNNECESSARY_MAYBE_SIZEDS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple bounds) |
| `NEEDLESS_OPTION_AS_DEREF` | `UNNECESSARY_OPTION_AS_DEREFS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple calls) |
| `NEEDLESS_OPTION_TAKE` | `UNNECESSARY_OPTION_TAKES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple calls) |
| `NEEDLESS_PARENS_ON_RANGE_LITERALS` | `UNNECESSARY_PARENS_ON_RANGE_LITERALS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_PASS_BY_REF_MUT` | `UNNECESSARY_PASS_BY_REF_MUTS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple parameters) |
| `NEEDLESS_PASS_BY_VALUE` | `UNNECESSARY_PASS_BY_VALUES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple parameters) |
| `NEEDLESS_PUB_SELF` | `UNNECESSARY_PUB_SELFS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple modifiers) |
| `NEEDLESS_QUESTION_MARK` | `UNNECESSARY_QUESTION_MARKS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple uses) |
| `NEEDLESS_RANGE_LOOP` | `UNNECESSARY_RANGE_LOOPS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple loops) |
| `NEEDLESS_RAW_STRINGS` | `UNNECESSARY_RAW_STRINGS` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_RAW_STRING_HASHES` | `UNNECESSARY_RAW_STRING_HASHES` | Uses 'needless' instead of 'unnecessary' (already plural) |
| `NEEDLESS_RETURN` | `UNNECESSARY_RETURNS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple statements) |
| `NEEDLESS_RETURN_WITH_QUESTION_MARK` | `UNNECESSARY_RETURNS_WITH_QUESTION_MARK` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple returns) |
| `NEEDLESS_SPLITN` | `UNNECESSARY_SPLITNS` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple calls) |
| `NEEDLESS_UPDATE` | `UNNECESSARY_UPDATES` | Uses 'needless' instead of 'unnecessary'; plural form (checks multiple updates) |
| `NEVER_LOOP` | `NEVER_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `NONMINIMAL_BOOL` | `NONMINIMAL_BOOLS` | Should use plural form (guideline 3) - checks multiple expressions |
| `NON_ASCII_LITERAL` | `NON_ASCII_LITERALS` | Should use plural form (guideline 3) - 'literal' is a countable noun |
| `OPTION_OPTION` | `OPTION_OPTIONS` | Should use plural form (guideline 3) - 'option' is a countable noun |
| `OP_REF` | `OP_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `OR_FUN_CALL` | `OR_FUN_CALLS` | Should use plural form (guideline 3) - 'call' is a countable noun |
| `PRINTLN_EMPTY_STRING` | `PRINTLN_EMPTY_STRINGS` | Should use plural form (guideline 3) - 'string' is a countable noun |
| `PRINT_LITERAL` | `PRINT_LITERALS` | Should use plural form (guideline 3) - 'literal' is a countable noun |
| `PTR_AS_PTR` | `PTR_AS_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `PTR_CAST_CONSTNESS` | `PTR_CAST_CONSTNESSES` | Should use plural form (guideline 3) - 'constness' is a countable noun |
| `PTR_EQ` | `PTRS_EQ` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `PTR_OFFSET_WITH_CAST` | `PTR_OFFSET_WITH_CASTS` | Should use plural form (guideline 3) - 'cast' is a countable noun |
| `RC_BUFFER` | `RC_BUFFERS` | Should use plural form (guideline 3) - 'buffer' is a countable noun |
| `READONLY_WRITE_LOCK` | `READONLY_WRITE_LOCKS` | Should use plural form (guideline 3) - 'lock' is a countable noun |
| `READ_ZERO_BYTE_VEC` | `READ_ZERO_BYTE_VECS` | Should use plural form (guideline 3) - 'vec' is a countable noun |
| `REDUNDANT_ALLOCATION` | `REDUNDANT_ALLOCATIONS` | Should use plural form (guideline 3) - 'allocation' is a countable noun |
| `REDUNDANT_CLOSURE_CALL` | `REDUNDANT_CLOSURE_CALLS` | Should use plural form (guideline 3) - 'call' is a countable noun |
| `REDUNDANT_PUB_CRATE` | `REDUNDANT_PUB_CRATES` | Should use plural form (guideline 3) - 'crate' is a countable noun |
| `REF_AS_PTR` | `REF_AS_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `REF_OPTION` | `REF_OPTIONS` | Should use plural form (guideline 3) - 'option' is a countable noun |
| `REF_OPTION_REF` | `REF_OPTION_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `REPLACE_BOX` | `REPLACE_BOXES` | Should use plural form (guideline 3) - 'box' is a countable noun |
| `RESERVE_AFTER_INITIALIZATION` | `RESERVE_AFTER_INITIALIZATIONS` | Should use plural form (guideline 3) - 'initialization' is a countable noun |
| `RESULT_MAP_OR_INTO_OPTION` | `RESULT_MAP_OR_INTO_OPTIONS` | Should use plural form (guideline 3) - 'option' is a countable noun |
| `SAME_FUNCTIONS_IN_IF_CONDITION` | `SAME_FUNCTIONS_IN_IF_CONDITIONS` | Should use plural form (guideline 3) - 'condition' is a countable noun |
| `SAME_NAME_METHOD` | `SAME_NAME_METHODS` | Should use plural form (guideline 3) - 'method' is a countable noun |
| `SELF_ASSIGNMENT` | `SELF_ASSIGNMENTS` | Should use plural form (guideline 3) - 'assignment' is a countable noun |
| `SHORT_CIRCUIT_STATEMENT` | `SHORT_CIRCUIT_STATEMENTS` | Should use plural form (guideline 3) - 'statement' is a countable noun |
| `SINGLE_CALL_FN` | `SINGLE_CALLS_FN` | Should use plural form (guideline 3) - 'call' is a countable noun |
| `SINGLE_ELEMENT_LOOP` | `SINGLE_ELEMENT_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `SINGLE_MATCH` | `SINGLE_MATCHES` | Should use plural form (guideline 3) - 'match' is a countable noun |
| `SIZE_OF_REF` | `SIZE_OF_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `SLOW_VECTOR_INITIALIZATION` | `SLOW_VECTOR_INITIALIZATIONS` | Should use plural form (guideline 3) - 'initialization' is a countable noun |
| `STRING_SLICE` | `STRING_SLICES` | Should use plural form (guideline 3) - 'slice' is a countable noun |
| `STR_TO_STRING` | `STR_TO_STRINGS` | Should use plural form (guideline 3) - 'string' is a countable noun |
| `SWAP_PTR_TO_REF` | `SWAP_PTR_TO_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `TEMPORARY_ASSIGNMENT` | `TEMPORARY_ASSIGNMENTS` | Should use plural form (guideline 3) - 'assignment' is a countable noun |
| `TESTS_OUTSIDE_TEST_MODULE` | `TESTS_OUTSIDE_TEST_MODULES` | Should use plural form (guideline 3) - 'module' is a countable noun |
| `TRANSMUTE_PTR_TO_PTR` | `TRANSMUTE_PTR_TO_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |
| `TRANSMUTE_PTR_TO_REF` | `TRANSMUTE_PTR_TO_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `TRIVIALLY_COPY_PASS_BY_REF` | `TRIVIALLY_COPY_PASS_BY_REFS` | Should use plural form (guideline 3) - 'ref' is a countable noun |
| `TYPE_ID_ON_BOX` | `TYPE_ID_ON_BOXES` | Should use plural form (guideline 3) - 'box' is a countable noun |
| `UNCHECKED_TIME_SUBTRACTION` | `UNCHECKED_TIME_SUBTRACTIONS` | Should use plural form (guideline 3) - 'subtraction' is a countable noun |
| `UNINIT_VEC` | `UNINIT_VECS` | Should use plural form (guideline 3) - 'vec' is a countable noun |
| `UNNECESSARY_CAST` | `UNNECESSARY_CASTS` | Should use plural form (guideline 3) - 'cast' is a countable noun |
| `UNNECESSARY_MAP_ON_CONSTRUCTOR` | `UNNECESSARY_MAP_ON_CONSTRUCTORS` | Should use plural form (guideline 3) - 'constructor' is a countable noun |
| `UNNECESSARY_OPERATION` | `UNNECESSARY_OPERATIONS` | Should use plural form (guideline 3) - 'operation' is a countable noun |
| `UNNECESSARY_SAFETY_COMMENT` | `UNNECESSARY_SAFETY_COMMENTS` | Should use plural form (guideline 3) - 'comment' is a countable noun |
| `UNNECESSARY_STRUCT_INITIALIZATION` | `UNNECESSARY_STRUCT_INITIALIZATIONS` | Should use plural form (guideline 3) - 'initialization' is a countable noun |
| `UNREADABLE_LITERAL` | `UNREADABLE_LITERALS` | Should use plural form (guideline 3) - 'literal' is a countable noun |
| `UNSOUND_COLLECTION_TRANSMUTE` | `UNSOUND_COLLECTION_TRANSMUTES` | Should use plural form (guideline 3) - 'transmute' is a countable noun |
| `USED_UNDERSCORE_ITEMS` | `USED_UNDERSCORES` | Has redundant '_items' suffix (guideline 2); plural form |
| `USELESS_ASREF` | `UNUSED_ASREFS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple calls) |
| `USELESS_ATTRIBUTE` | `UNUSED_ATTRIBUTES` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple attributes) |
| `USELESS_CONCAT` | `UNUSED_CONCATS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple calls) |
| `USELESS_CONVERSION` | `UNUSED_CONVERSIONS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple calls) |
| `USELESS_FORMAT` | `UNUSED_FORMATS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple calls) |
| `USELESS_LET_IF_SEQ` | `UNUSED_LET_IF_SEQS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple patterns) |
| `USELESS_NONZERO_NEW_UNCHECKED` | `UNUSED_NONZERO_NEW_UNCHECKEDS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple calls) |
| `USELESS_TRANSMUTE` | `UNUSED_TRANSMUTES` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple calls) |
| `USELESS_VEC` | `UNUSED_VECS` | Uses 'useless' instead of 'unused' (guideline 4); plural form (checks multiple constructions) |
| `VEC_BOX` | `VEC_BOXES` | Should use plural form (guideline 3) - 'box' is a countable noun |
| `VERBOSE_BIT_MASK` | `VERBOSE_BIT_MASKS` | Should use plural form (guideline 3) - 'mask' is a countable noun |
| `WHILE_IMMUTABLE_CONDITION` | `WHILE_IMMUTABLE_CONDITIONS` | Should use plural form (guideline 3) - 'condition' is a countable noun |
| `WHILE_LET_LOOP` | `WHILE_LET_LOOPS` | Should use plural form (guideline 3) - 'loop' is a countable noun |
| `WHILE_LET_ON_ITERATOR` | `WHILE_LET_ON_ITERATORS` | Should use plural form (guideline 3) - 'iterator' is a countable noun |
| `WRITELN_EMPTY_STRING` | `WRITELN_EMPTY_STRINGS` | Should use plural form (guideline 3) - 'string' is a countable noun |
| `WRITE_LITERAL` | `WRITE_LITERALS` | Should use plural form (guideline 3) - 'literal' is a countable noun |
| `WRONG_TRANSMUTE` | `WRONG_TRANSMUTES` | Should use plural form (guideline 3) - 'transmute' is a countable noun |
| `ZERO_PREFIXED_LITERAL` | `ZERO_PREFIXED_LITERALS` | Should use plural form (guideline 3) - 'literal' is a countable noun |
| `ZERO_PTR` | `ZERO_PTRS` | Should use plural form (guideline 3) - 'ptr' is a countable noun |

## Lints Kept Singular (Examples)

The following categories of lints are appropriately kept singular:

### Macros
- `DBG_MACRO` - refers to the `dbg!` macro itself
- `TODO_MACRO` - refers to the `todo!` macro itself
- `PRINT_LITERAL` - refers to printing pattern

### Traits
- `DEFAULT_TRAIT_ACCESS` - refers to the `Default` trait concept
- `FUTURE_NOT_SEND` - refers to the `Send` trait bound

### Patterns and Concepts
- `ASSIGN_OP_PATTERN` - refers to the assignment operator pattern
- `MODULE_STYLE` - refers to module organization style
- `ARBITRARY_SOURCE_ITEM_ORDERING` - refers to ordering concept
- `COGNITIVE_COMPLEXITY` - refers to complexity metric

## Lints Requiring Further Review

The following lints may have names that don't clearly state the bad thing being checked for:

| Current Name | What it checks | Issue | Suggested Alternative |
|--------------|----------------|-------|----------------------|
| `ALLOW_ATTRIBUTES` | Usage of `#[allow]` instead of `#[expect]` | Name suggests allowing is bad | Consider `MISSING_EXPECT_ATTRIBUTES` |
| `ALLOW_ATTRIBUTES_WITHOUT_REASON` | `#[allow]` attributes without a reason parameter | Confusing nested "allow" | Consider `MISSING_ALLOW_REASONS` |
| `USE_DEBUG` | Usage of Debug formatting in user-facing output | Name suggests positive action | Consider `DEBUG_IN_USER_OUTPUTS` |
| `USE_SELF` | Unnecessary repetition of struct name instead of `Self` | Name suggests positive action | Consider `UNNECESSARY_STRUCT_NAMES` |

**Note**: These suggestions use plural forms where appropriate.


## Rationale

### Why 'needless' should be 'unnecessary'

The RFC 0344 guidelines don't mention "needless" as a standard term. The term "unnecessary" is more standard in technical writing.

### Why 'useless' should be 'unused'

Per guideline 4, lints that catch "unnecessary, unused, or useless aspects of code should use the term `unused`".

### Why comprehensive noun-based pluralization

Per guideline 3, when a lint applies to a specific grammatical class (countable nouns), it should use the plural form. This makes lint names more accurate:
- `UNNECESSARY_BORROWS` - can find many unnecessary borrows
- `AWAIT_HOLDING_LOCKS` - can detect multiple lock-holding issues
- `CAST_PTR_ALIGNMENTS` - checks multiple pointer cast alignments

This makes `#[allow(unnecessary_borrows)]` read correctly as "allow (multiple) unnecessary borrows".

## Examples

Before:
```rust
#[allow(needless_borrow)]  // Singular
let x = &value;

#[allow(await_holding_lock)]  // Singular
let guard = mutex.lock().await;
```

After:
```rust
#[allow(unnecessary_borrows)]  // Plural
let x = &value;

#[allow(await_holding_locks)]  // Plural
let guard = mutex.lock().await;
```

## Implementation Notes

- These are proposed renames only and require community discussion
- Actual implementation would require updating lint declarations, tests, and documentation
- Old lint names should be deprecated with clear migration paths
- Consider using lint group aliases for backward compatibility
- Update all documentation, including the Clippy book
