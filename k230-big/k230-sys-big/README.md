Generate FFI
comm
bindgen comm.h -o src/comm.rs -- -I./k230_sdk/src/big/mpp/include/

big
bindgen big.h -o src/gpio.rs --ctypes-prefix "cty" \
  -- --sysroot=k230_sdk/toolchain/riscv64-linux-musleabi_for_x86_64-pc-linux-gnu/sysroot \
   -I./k230_sdk/src/big/rt-smart/kernel/include \
     -I./k230_sdk/src/big/rt-smart/kernel/bsp/maix3/include \
     -I./k230_sdk/src/big/rt-smart/kernel/bsp/maix3/board/interdrv/gpio \
     -include ./mock_riscv.h
