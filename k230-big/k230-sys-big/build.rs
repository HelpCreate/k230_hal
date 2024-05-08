use build_target;
use std::env;

fn main() {
    let target = build_target::target_triple().unwrap();

    enum TARGET {
        LittleCore,
        BigCore,
    }
    let target = match target.as_str() {
        "riscv64gc-unknown-linux-gnu" => TARGET::LittleCore,
        "riscv64gc-unknown-linux-musl" => TARGET::BigCore,
        _ => panic!("This crate should not be built for the host architecture"),
    };

    let sdk_path = env::var("K230_SDK_PATH").expect("Env K230_SDK_PATH not set");

}
