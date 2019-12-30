# ligen-devenv
Ligen Development Environment

## Testing

1. `cargo build`
2. `cargo clean -p geom` because in the first time it ran, the dlls weren't available
3. `cargo build -p geom`
4. `find target/debug/ligen -type f` to list the generated artifacts
