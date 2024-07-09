use std::{
    thread::{self, sleep},
    time::Duration,
};

use k230_hal::ipcmsg::{k_ipcmsg_message_t, Message, Service};

unsafe extern "C" fn handle_message1(id: i32, msg: *mut k_ipcmsg_message_t) {
    println!("got message");
}

unsafe extern "C" fn handle_message2(id: i32, msg: *mut k_ipcmsg_message_t) {}

unsafe extern "C" fn handel_response(message: *mut k_ipcmsg_message_t) {
    let response = Message::new_dont_drop(message);
    println!("{:?}", std::str::from_utf8(&response.get_body()).unwrap());
    println!("{:?}", response.get_module_id());
    println!("{:?}", response.get_cmd_id());
}

fn main() {
    println!("101");
    let service1 = Service::new(201, "Test1", false).expect("Failed to add service 1");
    let service2 = Service::new(202, "Test2", true).expect("Failed to add service 2");

    let connection1 = service1
        .try_connect(Some(handle_message1))
        .expect("Failed to connect service 1");
    let connection2 = service2
        .try_connect(Some(handle_message2))
        .expect("Failed to connect service 2");

    thread::spawn(move || {
        connection1.start();
    });

    loop {
        let message = Message::create(25, 0, *b"hello");
        connection1.send_response_async(message, Some(handel_response)).unwrap();
        sleep(Duration::new(1, 0));
    }
}

