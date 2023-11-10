use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::thread;
use once_cell::sync::OnceCell;


fn handle_stream(mut stream: TcpStream) {

    // let ival = 123i32;
    // let ival_bytes = ival.to_be_bytes();  // bytes length is 4
    // let sval_bytes = "12345".as_bytes();  // bytes length is 5
    // let mut total_bytes: Vec<u8> = vec![];
    // total_bytes.extend(ival_bytes);
    // total_bytes.extend(sval_bytes.clone());
    // println!("total_bytes lenght is {:?}", total_bytes.len());  // length is 9

    let mut data = [0 as u8; 64];
    while match stream.read(&mut data) {
        Ok(size) => {
            let msg_byte = &data[0..size];
            let message = String::from_utf8(msg_byte.to_vec()).unwrap();
            if message == "" || message == "Close" {
                stream.shutdown(Shutdown::Write).unwrap();
                false
            } else {
                unsafe {
                    let map = EXT_CONSTANT.get_mut().unwrap();
                    map.insert(String::from("ck1"), message.clone());
                    println!("Server reset value of Ext Constant is {:?}", message);
                }
                env::set_var("k1", message.clone());
                println!("Server reset value of k1 {:?}\n", message);
                let replay_txt = "Success".as_bytes();
                stream.write(replay_txt).unwrap();
                true
            }
        },
        Err(e) => {
            println!("An error occurred, terminating connection with {:?} {:?}", stream.peer_addr().unwrap(), e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn start_server(address: String) {

    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Server listen at address {:?}", address.clone());
    for stream in listener.incoming() {
        let k1 = env::var("k1").unwrap();
        println!("Variable k1 current value is {:?}", k1);

        unsafe {
            let map = EXT_CONSTANT.get().unwrap();
            println!("Ext Constant current value is {:?}", map.get("ck1").unwrap());
        }
        match stream {
            Ok(stream) => {
                println!("Get new connection {:?}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_stream(stream);
                });
            },
            Err(e) => {
                println!("Tcp Server Error: {:?}", e);
            }
        }
    }
    drop(listener);
}

fn start_client(address: String, message: String) {

    match TcpStream::connect(address.clone()) {
        Ok(mut stream) => {
            println!("Successfully connect {:?}", address.clone());

            let msg = message.as_bytes();
            stream.write(msg).unwrap();
            println!("Waiting for relay");

            let mut data = [0 as u8; 7];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text_byte = &data;
                    let text = String::from_utf8(text_byte.to_vec()).unwrap();
                    println!("Get relay {:?}", text)
                },
                Err(e) => {
                    println!("Fail to receive data: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("Fail to connect: {:?}", e);
        }
    }
}

pub static mut EXT_CONSTANT: OnceCell<HashMap<String, String>> = OnceCell::new();

// cargo run --bin about_std_tcp server 127.0.0.1:12345
// cargo run --bin about_std_tcp client 127.0.0.1:12345
fn main() {

    let args: Vec<String> = env::args().collect();
    let part_type = args[1].clone();

    let mut address = String::from("127.0.0.1:12345");
    if args.len() > 2 {
        address = args[2].clone();
    }
    
    let mut message = String::from("hello");
    if args.len() > 3 {
        message = args[3].clone();
    }

    env::set_var("k1", "Init Value");
    
    let mut values: HashMap<String, String> = HashMap::new();
    values.insert(String::from("ck1"), String::from("Origin OnceCell Value"));
    unsafe {
        let _ = EXT_CONSTANT.set(values);
    }

    if part_type.to_lowercase() == "server" {
        start_server(address);
    } else if part_type.to_lowercase() == "client" {
        start_client(address, message);
    } else {
        println!("Invalid part of server or client")
    }
}