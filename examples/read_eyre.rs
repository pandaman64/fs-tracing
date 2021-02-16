#![deny(future_incompatible, rust_2018_idioms, trivial_casts, unsafe_code)]

mod globals;

use fs_tracing as fs;

fn main() -> color_eyre::eyre::Result<()> {
    globals::install();
    color_eyre::install().unwrap();

    fs::read("/not_exist")?;
    Ok(())
}
