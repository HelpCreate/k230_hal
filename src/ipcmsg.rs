use k230_sys::k_ipcmsg_message_t;

pub fn send(receiver: fn()) {}

use std::{ffi::c_char, ptr};

use k230_sys::{
    self, k_ipcmsg_connect_t, k_ipcmsg_handle_fn_ptr, k_s32, kd_ipcmsg_add_service,
    kd_ipcmsg_connect,
};


pub struct Connection {
    port: u16,
    name: String,
}
impl Connection {
    pub fn new(port: u16, name: String, higher_priority: bool) -> Self {
        assert!(port < 512);
        let connection_config = k_ipcmsg_connect_t {
            u32RemoteId: 8,
            u32Port: port.into(),
            u32Priority: higher_priority.into(),
        };
        unsafe {
            if kd_ipcmsg_add_service(
                name.as_ptr() as *const c_char,
                &connection_config as *const k_ipcmsg_connect_t,
            ) != 0
            {
                panic!("Failed init");
            }
        }

        Self { port, name }
    }
    pub fn connect(&self, callback: k_ipcmsg_handle_fn_ptr) {
        // callback

        let connection_handler: *mut k_s32 = ptr::null_mut();
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
