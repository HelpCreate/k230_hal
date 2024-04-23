Generate FFI
comm
bindgen comm.h -o src/comm.rs -- -I./k230_sdk/src/big/mpp/include/

big
bindgen big.h -o src/big.rs -- --sysroot=k230_sdk/toolchain/riscv64-linux-musleabi_for_x86_64-pc-linux-gnu/sysroot

