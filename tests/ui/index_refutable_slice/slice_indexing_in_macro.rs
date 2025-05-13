#![deny(clippy::index_refutable_slice)]
#![allow(clippy::uninlined_format_args)]

macro_rules! if_let_slice_macro {
    () => {
        // This would normally be linted
        let slice: Option<&[u32]> = Some(&[1, 2, 3]);
        if let Some(slice) = slice {
            println!("{}", slice[0]);
        }
    };
}

fn main() {
    // Don't lint this
    if_let_slice_macro!();

    // Do lint this
    let slice: Option<&[u32]> = Some(&[1, 2, 3]);
    if let Some(slice) = slice {
        //~^ ERROR: this binding can be a slice pattern to avoid indexing
        println!("{}", slice[0]);
    }
}
