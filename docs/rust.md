<!-- @format -->

# Rust

## `crate` and `modules`

- [Defining modules to control scope and privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- [`crate` and `modules`](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/crates-and-modules.html)

`crate` is synonymous with a library or package in other languages. Hence "Cargo" as the name of Rust's package management tool: you ship your crates to others with Cargo. Crates can produce:
1. an executable(`src/main.rs` binary)
2. a shared library(`src/lib.rs` library)

Each `crate` has an implicit root `module` that contains the code for that crate. You can then define a tree of sub-modules under that root module. Modules allow you to partition your code within the crate itself.

```shell
phrases # name of crate
├── english # module
│   ├── farewells # module
│   └── greetings # module
└── korean # module
    ├── farewells # module
    └── greetings # module
```

```rust
mod english {
    mod greetings {}
    mod farewells {}
}
mod korean {
    mod greetings {}
    mod farewells {}
}
```

### [crate](https://doc.rust-lang.org/rust-by-example/crates.html)

[`crate`는 compilation, linking, versioning, distribution, 그리고 runtime loading 단위](https://doc.rust-lang.org/reference/crates-and-source-files.html)

`rustc some_file.rs` 실행 시, `some_file.rs`는 `crate` 파일
`some_file.rs`가 `mod`로 선언된 코드를 갖고 있다면, 컴파일러가 실행되기 전에 모듈 파일의 컨텐츠는 `crate` 파일에서 `mod`가 선언된 곳에 삽입된다.
즉, 모듈은 개별적으로 컴파일되지 않으며, `crate`만 컴파일된다

`crate`는 binary 또는 library로 컴파일될 수 있다. `rustc`는 `crate`로 바이너리를 만들며, 이는 `--crate-type` 플래그로 조절할 수 있다.

[`crate root`](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)는 라이브러리인 경우 `src/lib.rs`, 바이너리인 경우 `src/main.rs`

### [modules](https://doc.rust-lang.org/reference/items/modules.html)

> It is encouraged to use the new naming convention as it is more consistent, and avoids having many files named `mod.rs` within a project.

[module and file hierarchy](https://doc.rust-lang.org/reference/items/modules.html)

- module path: `crate`
- file path: `lib.rs`
- file content: `mod util;`

- module path: `crate::util`
- file path: `util.rs`
- file content: `mod config;`

- module path: `crate::util::config.rs`
- file path: `util/config.rs`
- file content: NONE
