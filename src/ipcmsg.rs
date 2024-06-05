use std::{ffi::c_char, ptr};

use k230_sys::{
    self, k_ipcmsg_connect_t, k_ipcmsg_handle_fn_ptr, k_s32, kd_ipcmsg_add_service,
    kd_ipcmsg_connect, kd_ipcmsg_del_service, kd_ipcmsg_disconnect, k_ipcmsg_message_t,
};

pub struct Service {
    port: u16,
    name: String,
}
impl Service {
    pub fn new(port: u16, name: String, higher_priority: bool) -> Self {
        assert!(port < 512);
        let connection_config = k_ipcmsg_connect_t {
            u32RemoteId: 0,
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
    pub fn connect(&mut self, message_handel: k_ipcmsg_handle_fn_ptr) -> Connection {
        let msg_handler : k_ipcmsg_handle_fn_ptr = Some(handel);

        unsafe extern "C" fn handel(s32id : i32,pstMsg : *mut k_ipcmsg_message_t) {
            
        }


        let connection_handler: *mut k_s32 = ptr::null_mut();
        unsafe {
            kd_ipcmsg_connect(
                connection_handler as *mut k_s32,
                self.name.as_ptr() as *const c_char,
                msg_handler,
            );
        }
        

        return Connection::new(connection_handler as i32)
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        unsafe {
            kd_ipcmsg_del_service(self.name.as_ptr() as *const c_char);
        }
    }
    
}
pub struct Connection {
    communication_id: i32,
}
impl Connection {
    pub fn new(communication_id: i32) -> Self {
        Self {
            communication_id,
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
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            kd_ipcmsg_disconnect(self.communication_id);
        }
    }
    
}


struct Message; 

