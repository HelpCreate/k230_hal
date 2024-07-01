use std::env;
use build_target;

fn main() {
    let target = build_target::target_triple().unwrap();

    match target.as_str() {
        "riscv64gc-unknown-linux-gnu" => (),
        "riscv64gc-unknown-linux-musl" => (),
        _ => panic!("This crate should not be built for the host architecture"),
    };

    let sdk_path = env::var("K230_SDK_PATH").expect("Env K230_SDK_PATH not set");

    // Add the directory containing libipcmsg.a to the library search path
    println!("cargo:rustc-link-search={}/src/common/cdk/user/component/ipcmsg/host/lib", sdk_path);

    // Link against the ipcmsg library
    println!("cargo:rustc-link-lib=static=ipcmsg");
    // println!("cargo:rustc-link-arg=-T{}/src/common/cdk/user/component/ipcmsg/sample/sample_sender/linker_scripts/riscv64/link.lds",sdk_path);
}

