use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::thread;

fn handle_stream(mut stream: TcpStream) {

    let mut data = [0 as u8; 64];
    while match stream.read(&mut data) {
        Ok(size) => {
            let msg_byte = &data[0..size];
            let message = String::from_utf8(msg_byte.to_vec()).unwrap();
            if message == "" || message == "Close" {
                stream.shutdown(Shutdown::Write).unwrap();
                false
            } else {
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

    if part_type.to_lowercase() == "server" {
        start_server(address);
    } else if part_type.to_lowercase() == "client" {
        start_client(address, message);
    } else {
        println!("Invalid part of server or client")
    }
}