use std::{ffi::{c_char, c_void}, ptr, mem };

use k230_sys::{
    self, k_ipcmsg_connect_t, k_s32,
    kd_ipcmsg_add_service, kd_ipcmsg_connect, kd_ipcmsg_del_service, kd_ipcmsg_disconnect, kd_ipcmsg_create_message, kd_ipcmsg_create_resp_message, kd_ipcmsg_destroy_message, kd_ipcmsg_send_only, kd_ipcmsg_send_sync, kd_ipcmsg_send_async, kd_ipcmsg_run,
};

pub use k230_sys::k_ipcmsg_message_t;

pub struct Service {
    name: &'static str
}
impl Service {
    pub fn new(port: u16, name: &'static str, higher_priority: bool) -> Result<Self,()> {
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
                return Err(());
            }
        }
        Ok(Self { name })
    }
    pub fn connect(&self, message_handel:Option<unsafe extern "C" fn(i32, *mut k_ipcmsg_message_t)>) -> Result<Connection,()> { 
        let connection_handler: *mut k_s32 = ptr::null_mut();
        unsafe {
            if 0 != kd_ipcmsg_connect(
                connection_handler as *mut k_s32,
                self.name.as_ptr() as *const c_char,
                message_handel,
            ) {
                return Err(());
            }
        }

        return Ok(Connection::new(connection_handler as i32));
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
        Self { communication_id }
    }
    pub fn start(self) {
        unsafe {kd_ipcmsg_run(self.communication_id);}
    }
    pub fn send_only(self, mut message: Message) -> Result<(),()> {
        unsafe {
            if 0 != kd_ipcmsg_send_only(self.communication_id,&mut message.0) {return Err(())}
            else {return Ok(())}
        }

    }
    pub fn send_response_blocking(self,mut message: Message, until_timeout : u16) -> Result<Message,()> {
        unsafe {
            let response_message = ptr::null_mut();
            if 0 !=  kd_ipcmsg_send_sync(self.communication_id, &mut message.0, response_message, until_timeout as i32)
            { return Err(());} 
            else {return Ok(Message::new(**response_message))}
        }
    }
    pub fn send_response_async<F>(self,mut message: Message, handel_response : Option<unsafe extern "C" fn(*mut k230_sys::IPCMSG_MESSAGE_S)>) -> Result<(), ()>{ 
        unsafe {
            if 0 != kd_ipcmsg_send_async(self.communication_id, &mut message.0,handel_response) {return Err(())}
            else {return Ok(())}

        }
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            kd_ipcmsg_disconnect(self.communication_id);
        }
    }
}

pub struct Message(pub k230_sys::k_ipcmsg_message_t);
impl Message {
    pub fn new(message: k230_sys::k_ipcmsg_message_t) -> Self {
        Self(message)
    }
    pub fn create<T>(module_id : u32,cmd_id : u32, mut body : T) -> Self {
        unsafe {
            let body_ptr: *mut T = &mut body;
         let message = kd_ipcmsg_create_message(module_id, cmd_id,body_ptr as *mut c_void,mem::size_of::<T>() as u32);   
            Self(*message)
        }
    }
    pub fn create_response<T>(mut original_message : Message, message_handel : i32, mut body : T) -> Self { 
        unsafe {
            let body_ptr: *mut T = &mut body;
         let message = kd_ipcmsg_create_resp_message(&mut original_message.0,message_handel, body_ptr as *mut c_void,mem::size_of::<T>() as u32);   
            Self(*message)

        }
        
    }
        pub unsafe fn body_to_type<T>(self) -> T {
        assert_eq!(self.0.u32BodyLen as usize, std::mem::size_of::<T>());
        let body_ptr = self.0.pBody as *mut T;
        body_ptr.read()
    }
}
impl Drop for Message {
    fn drop(&mut self) {
         unsafe {
             kd_ipcmsg_destroy_message(&mut self.0);
         }
    }
    
}
