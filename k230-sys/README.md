Generate FFI
bindgen wrapper.h -o src/comm.rs -- -I./k230_sdk/src/big/mpp/include/
