use std::ffi::{c_char, c_void};

use k230_sys::{
    self, k_ipcmsg_connect_t, k_s32, kd_ipcmsg_add_service, kd_ipcmsg_connect,
    kd_ipcmsg_create_message, kd_ipcmsg_create_resp_message, kd_ipcmsg_del_service,
    kd_ipcmsg_destroy_message, kd_ipcmsg_disconnect, kd_ipcmsg_is_connect, kd_ipcmsg_run,
    kd_ipcmsg_send_async, kd_ipcmsg_send_only, kd_ipcmsg_send_sync, kd_ipcmsg_try_connect,
};

pub use k230_sys::k_ipcmsg_message_t;

pub struct Service {
    name: &'static str,
}
impl Service {
    pub fn new(port: u32, name: &'static str, higher_priority: bool) -> Result<Self, ()> {
        assert!(port < 512);
        let connection_config = k_ipcmsg_connect_t {
            u32RemoteId: 1,
            u32Port: port,
            u32Priority: higher_priority.into(),
        };
        unsafe {
            let status = kd_ipcmsg_add_service(
                name.as_ptr() as *const c_char,
                &connection_config as *const k_ipcmsg_connect_t,
            );
            if status != 0 {
                return Err(());
            }
        }
        Ok(Self { name })
    }

    pub fn connect(
        &self,
        message_handel: Option<unsafe extern "C" fn(i32, *mut k_ipcmsg_message_t)>,
    ) -> Result<Connection, i32> {
        let mut connection_handler: k_s32 = 0;
        unsafe {
            let status = kd_ipcmsg_connect(
                &mut connection_handler as *mut k_s32,
                self.name.as_ptr() as *const c_char,
                message_handel,
            ) as i32;
            if status != 0 {
                return Err(status);
            }
        }
        Ok(Connection::new(connection_handler as i32))
    }

    pub fn try_connect(
        &self,
        message_handel: Option<unsafe extern "C" fn(i32, *mut k_ipcmsg_message_t)>,
    ) -> Result<Connection, i32> {
        let mut connection_handler: k_s32 = 0;
        unsafe {
            let status = kd_ipcmsg_try_connect(
                &mut connection_handler as *mut k_s32,
                self.name.as_ptr() as *const c_char,
                message_handel,
            );
            if status != 0 {
                return Err(status);
            }
        }
        Ok(Connection::new(connection_handler as i32))
    }
}
impl Drop for Service {
    fn drop(&mut self) {
        unsafe {
            kd_ipcmsg_del_service(self.name.as_ptr() as *const c_char);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Connection {
    communication_id: i32,
}
impl Connection {
    pub fn new(communication_id: i32) -> Self {
        Self { communication_id }
    }
    pub fn start(&self) {
        unsafe {
            kd_ipcmsg_run(self.communication_id);
        }
    }
    pub fn is_connected(&self) -> bool {
        unsafe { kd_ipcmsg_is_connect(self.communication_id) != 0 }
    }
    pub fn send_only(&self, message: Message) -> Result<(), ()> {
        unsafe {
            if 0 != kd_ipcmsg_send_only(self.communication_id, message.0) {
                return Err(());
            } else {
                return Ok(());
            }
        }
    }
    pub fn send_response_blocking(
        &self,
        message: Message,
        until_timeout: u16,
    ) -> Result<Message, i32> {
        let mut response_message: *mut k_ipcmsg_message_t = std::ptr::null_mut();
        let status = unsafe {
            kd_ipcmsg_send_sync(
                self.communication_id,
                message.0,
                &mut response_message,
                until_timeout as i32,
            )
        };
        if status != 0 {
            Err(status)
        } else {
            Ok(Message::new(response_message))
        }
    }
    pub fn send_response_async(
        self,
        message: Message,
        handel_response: Option<unsafe extern "C" fn(*mut k230_sys::IPCMSG_MESSAGE_S)>,
    ) -> Result<(), ()> {
        unsafe {
            if 0 != kd_ipcmsg_send_async(self.communication_id, message.0, handel_response) {
                return Err(());
            } else {
                return Ok(());
            }
        }
    }
}
// impl Drop for Connection {
//    fn drop(&mut self) {
//         unsafe {
// kd_ipcmsg_disconnect(self.communication_id);
//        }
//    }
// }

pub struct Message(pub *mut k230_sys::k_ipcmsg_message_t, bool);
impl Message {
    pub fn new(message: *mut k230_sys::k_ipcmsg_message_t) -> Self {
        Self(message, true)
    }

    pub fn new_dont_drop(message: *mut k230_sys::k_ipcmsg_message_t) -> Self {
        Self(message, false)
    }

    pub fn create<const N: usize>(module_id: u32, cmd_id: u32, body: [u8; N]) -> Self {
        let body_ptr = body.as_ptr() as *const c_void;
        let body_len = body.len() as u32;

        // Call the external function to create the message
        let message =
            unsafe { kd_ipcmsg_create_message(module_id, cmd_id, body_ptr, body.len() as u32) };

        // Print the message pointer
        println!("message pointer: {:?}", message);

        // Check if the message creation was successful
        if message.is_null() {
            panic!("Failed to create message");
        }

        // Return the new Self instance
        Self(message, true)
    }
    pub fn create_response<const N: usize>(
        original_message: Message,
        message_handel: i32,
        mut body: [u8; N],
    ) -> Self {
        unsafe {
            let body_ptr = body.as_mut_ptr() as *mut c_void;
            let message = kd_ipcmsg_create_resp_message(
                original_message.0,
                message_handel,
                body_ptr,
                body.len() as u32,
            );
            Self(message, true)
        }
    }
    pub fn get_body(&self) -> Vec<u8> {
        let mut data = vec![];
        let message = unsafe { self.0.as_ref().unwrap() };
        let body = message.pBody as *const u8;
        for i in 0..message.u32BodyLen {
            data.push(unsafe { body.offset(i as isize).read() })
        }
        data
    }
    pub fn get_module_id(&self) -> u32 {
        unsafe { self.0.as_ref().unwrap().u32Module }
    }
    pub fn get_cmd_id(&self) -> u32 {
        unsafe { self.0.as_ref().unwrap().u32CMD }
    }
}
impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            if self.1 {
                kd_ipcmsg_destroy_message(self.0);
            }
        }
    }
}
