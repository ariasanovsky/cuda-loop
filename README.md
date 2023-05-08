# cuda-loop

## ptx

### Emitting ptx

To emit `target/nvptx64-nvidia-cuda/release/deps/kernel.ptx`, run

```powershell
cargo +nightly rustc --lib --target nvptx64-nvidia-cuda --release -- --emit asm
```

Use

```powershell
cp target/nvptx64-nvidia-cuda/release/deps/kernel.ptx kernels/
```

to relocate to `kernels/`.

For convenience:

```powershell
rm kernels/* ; cargo clean ; cargo +nightly rustc --lib --target nvptx64-nvidia-cuda --release -- --emit asm ; cp target/nvptx64-nvidia-cuda/release/deps/kernel.ptx ker
nels/ ; ls kernels/
```

See `lib.rs` and `device.rs` for comments on the kernel and device functions.

### Planned features

- macro to write kernel from device function and build script to emit ptx
- include input fields like in `https://github.com/coreylowman/cudarc/blob/main/examples/03-launch-kernel.rs`


### Running the mvp

Run with

```powershell
cargo run --release --example mvp --features cudarc
```

after emitting `ptx`.
