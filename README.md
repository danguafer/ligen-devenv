# ligen-devenv
Ligen Development Environment

## Testing

1. `git clone --recursive https://github.com/danguafer/ligen-devenv`
2. `cd ligen-devenv`
3. `cargo build` to compile the generators' dlls
4. `cargo test`
5. `find target/debug/ligen -type f` to list the generated artifacts

Read `ligen-c/tests/test.rs` and `ligen-cpp/tests/test.rs` to check which object is being generated and how `ligen` is used.
