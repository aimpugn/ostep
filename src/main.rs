#![allow(dead_code, unused_imports)]
// here `src/main.rs` is crate root of binary
use std::env;

// tell compiler to include the code it finds in `src/chapters.rs`
pub mod chapters;
pub mod util;

// use public modules from `src/chapters/introduction/mod.rs`
use chapters::introduction::*;
use util::args::{ArgsMap, Config};

fn main() {
    let _args = Config.parse();
    // cpu::print_and_sleep(_args.get("--param").unwrap().to_string());
    // memory::print_memory_and_sleep();
    // memory::alloc_then_print_and_sleep();
    // concurrency::wrongly_working_concurrency_program(100_000);
    persistence::write_hello_world();
}
