use k230_sys::k_ipcmsg_message_t;
pub fn send(receiver: fn()) {
}

use std::{ffi::c_char, ptr};

use k230_sys::{
    self, k_ipcmsg_connect_t, k_ipcmsg_handle_fn_ptr, k_s32, kd_ipcmsg_add_service,
    kd_ipcmsg_connect,
};

use super::WHICH_CORE; 



pub struct Connection {
    port: u16,
}
impl Connection {
    pub fn new(port: u16, higher_priority: bool) -> Self {
        assert!(port < 512);
        let connection_config = k_ipcmsg_connect_t {
            u32RemoteId: WHICH_CORE as u32,
            u32Port: port.into(),
            u32Priority: higher_priority.into(),
        };
        unsafe {
            if kd_ipcmsg_add_service(
                port as *const c_char,
                &connection_config as *const k_ipcmsg_connect_t,
            ) != 0
            {
                panic!("Failed init");
            }
        }

        Self { port }
    }
    pub fn connect(&self, (callbacks,id): (impl FnOnce(),u32)) { // TODO input params of the
        // callback

        extern "C" fn on_message_received(msg_id: k_s32, p_msg: *mut k_ipcmsg_message_t) {
            let msg = unsafe {
             p_msg.read()   
            };

            
     
            
        }
        let connection_handler: *mut k_s32 = ptr::null_mut();
        let callback: k_ipcmsg_handle_fn_ptr = Some(on_message_received);
        unsafe {
            kd_ipcmsg_connect(
                connection_handler as *mut k_s32,
                self.port as *const u8,
                callback,
            );
        }
    }
    pub fn send_only() {
        // TODO
    }
    pub fn send_response_blocking() {
        // TODO
    }
    pub fn send_response_() {
        // TODO
    }
}


