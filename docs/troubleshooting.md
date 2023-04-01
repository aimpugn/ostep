# Troubleshooting

## Blocking waiting for file lock on package cache

### 문제

```shell
❯ cargo install
    Blocking waiting for file lock on package cache

```

### 원인

캐시 문제인 것으로 보이며, [검색해 보니 그냥 클린](https://stackoverflow.com/a/49634470/8562273)하면 되는 것 같다

### 해결

```shell
❯ cargo clean
❯ cargo install
error: Using `cargo install` to install the binaries for the package in current working directory is no longer supported, use `cargo install --path .` instead. Use `cargo build` if you want to simply build the package.

❯ cargo install --path .
  Installing ostep v0.1.0 (/Users/rody/VscodeProjects/ostep)
    Updating crates.io index
  Downloaded libc v0.2.140
  Downloaded fork v0.1.21
  Downloaded 2 crates (674.0 KB) in 1.71s
   Compiling libc v0.2.140
   Compiling cfg-if v1.0.0
   Compiling autocfg v1.1.0
   Compiling ppv-lite86 v0.2.17
   Compiling pin-utils v0.1.0
   Compiling bitflags v1.3.2
   Compiling static_assertions v1.1.0
   Compiling memoffset v0.7.1
   Compiling getrandom v0.2.8
   Compiling nix v0.26.2
   Compiling fork v0.1.21
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling ostep v0.1.0 (/Users/rody/VscodeProjects/ostep)
    Finished release [optimized + debuginfo] target(s) in 6.01s
  Installing /Users/rody/.cargo/bin/ostep
   Installed package `ostep v0.1.0 (/Users/rody/VscodeProjects/ostep)` (executable `ostep`)
```
