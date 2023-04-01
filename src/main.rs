#![allow(dead_code, unused_imports)]
// here `src/main.rs` is crate root of binary
use std::{collections::HashMap, env};

// tell compiler to include the code it finds in `src/chapters.rs`
pub mod chapters;
pub mod util;

// use public modules from `src/chapters/introduction/mod.rs`
use chapters::{introduction::*, process};
use util::args::{ArgsMap, Config};

fn main() {
    let _args = Config.parse();
    lottery()
}

fn introduction() {
    // cpu::print_and_sleep(_args.get("--param").unwrap().to_string());
    // memory::print_memory_and_sleep();
    // memory::alloc_then_print_and_sleep();
    // concurrency::wrongly_working_concurrency_program(100_000);
    // persistence::write_hello_world();
}

fn process() {
    // process_run();
    // process::apis::test_fork_by_fork();
    // process::apis::test_fork_by_nix();
    process::apis::test_fork_by_nix_wait_child();
    // process::apis::test_exec_wc();
    // process::apis::test_exec_output_redirect();
    // process::apis::test_change_variable_from_child();
}

fn process_run() {
    // process
    let s = process::process_run::Scheduler {
        proc_info: HashMap::new(),
        process_switch_behavior: String::from(""),
        io_done_behavior: String::from(""),
        program: vec!["c7"],
        io_length: 0,
    };
    s.init();
    s.run();
}

fn lottery() {
    process::lottery::simple_lottery_scheduling();
}
