#![warn(clippy::collapsible_if)]
#![feature(let_chains)]

fn main() {
    if let Some(a) = Some(3) {
        if let Some(b) = Some(4) {
            let _ = a + b;
        }
    }
    //~^^^^^ collapsible_if

    if let Some(a) = Some(3) {
        if a + 1 == 4 {
            let _ = a;
        }
    }
    //~^^^^^ collapsible_if

    if Some(3) == Some(4).map(|x| x - 1) {
        if let Some(b) = Some(4) {
            let _ = b;
        }
    }
    //~^^^^^ collapsible_if
}
