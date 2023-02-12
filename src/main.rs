// here `src/main.rs` is crate root of binary

// tell compiler to include the code it finds in `src/chapters.rs`
pub mod chapters;
pub mod util;

use util::args::{ArgsMap, Config};

// use public modules from `src/chapters/introduction/mod.rs`
use crate::chapters::introduction::cpu;

fn main() {
    let _args = Config.parse();
    cpu::print_and_sleep(_args.get("--param").unwrap().to_string());
}
