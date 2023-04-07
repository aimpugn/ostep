<!-- @format -->

# commands

## [initialize new Cargo manifest](https://doc.rust-lang.org/cargo/commands/cargo-init.html)

```shell
CARGO_INIT_OPTIONS=(
    --bin # Create a package with a binary target
    --name ostep # set package name
)
cargo init $CARGO_INIT_OPTIONS
```

### [build cache](https://doc.rust-lang.org/cargo/guide/build-cache.html)

Cargo stores the output of a build into the `target` directory

```shell
├── debug # output for `dev` profile
│   ├── build
│   ├── deps
│   ├── examples
│   └── incremental
├── release # output for `release` profile
└── foo # output for `foo` profile when `--profile=foo`
```
