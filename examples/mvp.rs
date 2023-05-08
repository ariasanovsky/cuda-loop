use cudarc::{
    driver::{CudaDevice, DriverError, LaunchAsync, LaunchConfig},
    nvrtc::Ptx,
};

fn main() -> Result<(), DriverError> {
    let dev = CudaDevice::new(0)?;

    // You can load a function from a pre-compiled PTX like so:
    println!("loaoding...");
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
