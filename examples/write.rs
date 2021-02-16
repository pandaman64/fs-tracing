#![deny(future_incompatible, rust_2018_idioms, trivial_casts, unsafe_code)]

mod globals;

use fs_tracing as fs;

fn main() {
    globals::install();

    let e = fs::write("/not_exist", b"foo").unwrap_err();
    println!("Debug:\n{:?}", e);
    println!("Display:\n{}", e);
}
