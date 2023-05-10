use std::process::Command;
use std::io::{self, Write};
use std::fs;
use std::path::Path;

fn build_nvptx() -> io::Result<()> {
    let output = Command::new("cargo")
        .arg("+nightly")
        .arg("rustc")
        .arg("--lib")
        .arg("--target")
        .arg("nvptx64-nvidia-cuda")
        .arg("--release")
        .arg("--")
        .arg("--emit")
        .arg("asm")
        .output()?;
    
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    
    let target_dir = Path::new("target/nvptx64-nvidia-cuda/release");
    let kernel_ptx = target_dir.join("kernel.ptx");
    let kernels_dir = Path::new("kernels");
    
    if kernel_ptx.exists() {
        let dest = kernels_dir.join("kernel.ptx");
        fs::create_dir_all(kernels_dir)?;
        fs::copy(&kernel_ptx, &dest)?;
        println!("Copied {} to {}", kernel_ptx.display(), dest.display());
    }
    
    Ok(())
}

use cudarc::{
    driver::{CudaDevice, DriverError, LaunchAsync, LaunchConfig},
    nvrtc::Ptx,
};

fn main() -> Result<(), DriverError> {
    let dev = CudaDevice::new(0)?;

    build_nvptx().unwrap();

    // You can load a function from a pre-compiled PTX like so:
    println!("loading...");
    dev.load_ptx(Ptx::from_file("kernels/kernel.ptx"), "kernel", &["kernel"])?;
    println!("loaded!");

    // and then retrieve the function with `get_func`
    let f = dev.get_func("kernel", "kernel").unwrap();

    let a_host = [1, 2, 3];

    let a_dev = dev.htod_copy(a_host.into())?;
    let mut b_dev = a_dev.clone();

    let n = 3;
    let cfg = LaunchConfig::for_num_elems(n);
    unsafe { f.launch(cfg, (&mut b_dev, n as i32)) }?;

    let a_host_2 = dev.sync_reclaim(a_dev.clone())?;
    let b_host = dev.sync_reclaim(b_dev.clone())?;

    println!("a_host {a_host:?}");
    
    println!("b_host {b_host:?}");
    
    println!("a_host_2 {a_host_2:?}");
    
    Ok(())
}
