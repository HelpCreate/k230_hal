pub mod ipcmsg;

enum CORE {
    Little,
    Big,
}
const WHICH_CORE: CORE = if cfg!(musl) { CORE::Big } else { CORE::Little };
