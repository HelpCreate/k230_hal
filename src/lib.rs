// pub mod ipcmsg;
pub mod big;
pub mod small;

enum CORE {
    Little,
    Big,
}
const WHICH_CORE: CORE = if cfg!(musl) { CORE::Big } else { CORE::Little };
