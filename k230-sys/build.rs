use build_target;
use std::env;

fn main() {
    let target = build_target::target_triple().unwrap();

    enum TARGET {
        LittleCore,
        BigCore,
    }
    let target = match target.as_str() {
        "riscv32imac-unknown-none-elf" => TARGET::LittleCore,
        "riscv64imac-unknown-none-elf" => TARGET::BigCore,
        _ => panic!("This crate should not be built for the host architecture"),
    };
}
