#![allow(dead_code)]
#![warn(clippy::redundant_test_prefix)]

fn main() {
    // Normal function, no redundant prefix.
}

fn f1() {
    // Normal function, no redundant prefix.
}

fn test_f2() {
    // Has prefix, but no `#[test]` attribute, ignore.
}

#[test]
fn test_f3() {
    // Has prefix, has `#[test]` attribute.
    // Not within a `#[cfg(test)]`, ignore (by default, configurable -- see TOML tests).
}

#[cfg(test)]
#[test]
fn test_f4() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix
    //~| NOTE: `-D clippy::redundant-test-prefix` implied by `-D warnings`
    //~| HELP: to override `-D warnings` add `#[allow(clippy::redundant_test_prefix)]`

    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // No collision with other functions, should `test_` prefix be removed.
}

mod m1 {
    pub fn f5() {}
}

#[cfg(test)]
#[test]
fn test_f6() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix

    use m1::f5;

    f5();
    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // No collision, has function call, but it will not result in recursion.
}

mod m2 {
    use super::m1::f5;

    // FIX DOES NOT COMPILE
    #[cfg(test)]
    #[test]
    fn test_f5() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }
}

mod m3 {
    use super::m1::*;

    #[cfg(test)]
    #[test]
    fn test_f5() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
        eprintln!("Wrong one called");
    }

    // WRONG FUNCTION CALLED AFTER FIX
    fn some_func() {
        f5();
    }
}

mod m4 {
    mod m5 {
        pub fn f(_: i32) -> i32 {
            0
        }
    }

    use m5::*;

    // FIX DOES NOT COMPILE
    #[cfg(test)]
    #[test]
    fn test_f() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider function renaming (just removing `test_` prefix will cause a name conflict)
        let a = Some(3);
        let _ = a.map(f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_foo_with_call() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix

        main();
    }

    #[test]
    fn test_f1() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f2() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f3() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f4() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f5() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f6() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }
}

mod tests_no_annotations {
    use super::*;

    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_foo() {}

    #[test]
    fn test_foo_with_call() {
        main();
    }

    #[test]
    fn test_f1() {}

    #[test]
    fn test_f2() {}

    #[test]
    fn test_f3() {}

    #[test]
    fn test_f4() {}

    #[test]
    fn test_f5() {}

    #[test]
    fn test_f6() {}
}

// This test is inspired by real test in `clippy_utils/src/sugg.rs`.
// The `is_in_test_function()` checks whether any identifier within a given node's parents is
// marked with `#[test]` attribute. Thus flagging false positives when nested functions are
// prefixed with `test_`. Therefore `is_test_function()` has been defined in `clippy_utils`,
// allowing to select only functions that are immediately marked with `#[test]` annotation.
//
// This test case ensures that for such nested functions no error is emitted.
#[test]
fn not_op() {
    fn test_not(foo: bool) {
        assert!(foo);
    }

    // Use helper function
    test_not(true);
    test_not(false);
}
