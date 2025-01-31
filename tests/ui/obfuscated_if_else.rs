#![warn(clippy::obfuscated_if_else)]
#![allow(clippy::unnecessary_lazy_evaluations, clippy::unit_arg, clippy::unused_unit)]

fn main() {
    true.then_some("a").unwrap_or("b");
    true.then(|| "a").unwrap_or("b");

    let a = 1;
    (a == 1).then_some("a").unwrap_or("b");
    (a == 1).then(|| "a").unwrap_or("b");

    let partial = (a == 1).then_some("a");
    partial.unwrap_or("b"); // not lint

    let mut a = 0;
    true.then_some(a += 1).unwrap_or(());
    true.then_some(()).unwrap_or(a += 2);
}

fn issue11141() {
    // Parentheses are required around the left side of a binary expression
    let _ = true.then_some(40).unwrap_or(17) | 2;
    // Parentheses are required only for the leftmost expression
    let _ = true.then_some(30).unwrap_or(17) | true.then_some(2).unwrap_or(3) | true.then_some(10).unwrap_or(1);

    // Parentheses are not required around the right side of a binary expression
    let _ = 2 | true.then_some(40).unwrap_or(17);

    // Parentheses are not required for a cast
    let _ = true.then_some(42).unwrap_or(17) as u8;
    // Parentheses are not required for a deref
    let _ = *true.then_some(&42).unwrap_or(&17);
    // Parentheses are not required for a deref followed by a cast
    let _ = *true.then_some(&42).unwrap_or(&17) as u8;
}
