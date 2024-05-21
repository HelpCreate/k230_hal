export PATH="/home/neb/Github/k230_hal/k230-big/k230-sys-big/k230_sdk/toolchain/riscv64-linux-musleabi_for_x86_64-pc-linux-gnu/bin/:$PATH"
cargo +nightly build -Z build-std --target riscv64gc-unknown-linux-musl
