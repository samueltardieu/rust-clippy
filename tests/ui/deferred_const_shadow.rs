#![warn(clippy::deferred_const_shadow)]
#![allow(unused)]

use std::path::MAIN_SEPARATOR;
const MESSAGE: &str = "🦀";

fn main() {
    println!("{MESSAGE}");
    //~^ ERROR: using a shadowing constant defined later in the same scope
    const MESSAGE: &str = "Hello Rust!";

    // No lint here, as this appears after the constant definition
    println!("{MESSAGE}");

    // No lint here, as no constant with the same name is shadowed
    println!("{OTHER_MESSAGE}");
    const OTHER_MESSAGE: &str = "foobar";

    // No lint (because no shadow)
    println!("{MAIN_SEPARATOR}");
    const MAIN_SEPARATOR: &str = "::";
}
