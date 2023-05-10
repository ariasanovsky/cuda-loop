# cuda-loop

## Running and building PTX on-the-fly

The function

```rust
fn build_nvptx() -> io::Result<()> // in examples/mvp.rs
```

emits `kernels/kernel.ptx` from `src/lib.rs` by using `Command` and `+nightly` Rust.
This is neat, but not ideal for several reasons:

1. uses the path `src/lib.rs` instead of a general path argument
2. this is not the right way to use `lib.rs` in Rust 😂
3. is in `examples/mvp.rs` when it should be rehomed to `cudarc`. It's a cousin of `cudarc::nvrtc::Program::compile_ptx_with_opts`.

Convenience script:

```powershell
cargo clean ; rm kernels/* ; cargo run --release --example mvp --features cudarc
```

See `lib.rs` and `device.rs` for comments on the kernel and device functions.

## Planned features

1. macro to write common types of kernels from device functions -- type safety is tricky
2. incorporate `build_nvptx` into `cudarc` without too much extra plumbing
3. include input fields like in `https://github.com/coreylowman/cudarc/blob/main/examples/03-launch-kernel.rs`
4. what to do with all of the available [thread info](https://doc.rust-lang.org/stable/core/arch/nvptx/index.html):

```rust
_block_dim_x, _block_dim_y, _block_dim_z
_block_idx_x, _block_idx_y, _block_idx_z
_grid_dim_x, _grid_dim_y, _grid_dim_z
_syncthreads
_thread_idx_x, _thread_idx_y, _thread_idx_z
```

5. can we improve the `abi_ptx` [issues](https://github.com/rust-lang/rust/issues/38788)?
