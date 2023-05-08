// an ordinary (no_std?) Rust fn that calculates using only thread info
pub fn device(thread_id: i32, block_id: i32, n_threads: i32, grid_dim: i32) -> i32 {
    // your device function implementation
    // use tid, ctaid, ntid, and nctaid to determine what to write to the array
    thread_id + block_id + n_threads + grid_dim + 100000
}
