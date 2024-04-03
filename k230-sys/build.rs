use std::{fs, env};


#[cfg(target_arch = "riscv64")]
fn main() {
    


    
}
#[cfg(not(target_arch = "riscv64"))]
fn main() {
    panic!("This crate should not be built for the host architecture {}",  env::current_dir().unwrap().to_str().unwrap())
}

