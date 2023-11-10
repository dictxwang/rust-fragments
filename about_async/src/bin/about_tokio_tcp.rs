use docopt::Docopt;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use std::{net::SocketAddr, vec, io::Write};
use std::str;
use anyhow::Result;
use std::io;

// Write the Docopt usage string.
const USAGE: &'static str = "
Rust's package manager

Usage:
    cargo <command> [<args>...]
    cargo [options]

Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    --list           List installed commands
    -v, --verbose    Use verbose output

Some common cargo commands are:
    server      Run tcp server
    client      Run tcp client

See 'cargo help <command>' for more information on a specific command.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
    flag_list: bool,
    flag_verbose: bool,
}

#[derive(Debug, Deserialize)]
enum Command {
    Server,
    Client,
}

#[derive(Debug, Deserialize, Serialize)]
struct Resp {
    code: i32,
    msg: Option<String>,
    data: Option<String>,
}

async fn make_data_package(text: String) -> Vec<u8> {

    let text_bytes = text.as_bytes();
    let length = text_bytes.len() as i32;
    let mut data: Vec<u8> = vec![];
    data.extend(length.to_be_bytes());
    data.extend(text_bytes.clone());
    data
}

async fn handle_stream(mut stream: TcpStream, pair_addr: SocketAddr) -> Result<()> {

    stream.set_nodelay(true)?;
    println!("Get Connection With: {:?}", pair_addr.clone());

    loop {

        // Read
        // Firstly, get the header
        let mut header = [0 as u8; 4];
        stream.read(&mut header).await?;
        let length = i32::from_be_bytes(header);

        // Secondly, read all data
        let mut read_length = 0;
        let mut all_bytes: Vec<u8> = vec![];
        loop {
            let mut buf = [0 as u8; 1024];
            let size = stream.read(&mut buf).await? as i32;
            for index in 0..size {
                all_bytes.push(buf[index as usize]);
            }
            read_length += size;
            if read_length >= length {
                break;
            }
        }
        let receive_text = String::from_utf8(all_bytes)?;
        println!("Server Receive: {}", receive_text);

        if receive_text == "close" || receive_text == "exit" {
            println!("Close Connection With: {:?}", pair_addr);
            break;
        }

        // Write
        let resp = Resp {
            code: 0i32,
            msg: Some("OK".to_string()),
            data: Some(receive_text.clone()),
        };
        let echo = serde_json::to_string_pretty(&resp).unwrap();
        
        // let echo = echo.replace("\"", "'");
        let data = make_data_package(echo).await;

        stream.write(&data).await?;
        stream.flush().await?;
    }

    Ok(())

}

async fn start_server(addr: String) {

    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, pair_addr) = match listener.accept().await {
            Ok((stream, pair_addr)) => {
                (stream, pair_addr)
            },
            Err(e) => {
                println!("Accept Error: {:?}", e);
                continue;
            }
        };
        
        tokio::spawn(async move {
            match handle_stream(stream, pair_addr).await {
                Ok(_) => {},
                Err(e) => {
                    println!("Failed To Handle Stream: {:?}", e);
                }
            }
        });
    }
}

async fn start_client(addr: String) -> Result<()> {

    let mut stream = TcpStream::connect(addr).await.unwrap();
    println!("Welcome to Sando OP-Board");
    loop {
        // Wait
        print!("[sando-op]> ");
        io::stdout().flush()?;

        // Send
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read command");
        let text = String::from(input.trim());

        if text.is_empty() {
            continue;
        }
        let should_close = text == "close" || text == "exit";
        
        let data = make_data_package(text).await;
        stream.write(&data).await?;
        stream.flush().await?;

        if should_close {
            stream.shutdown().await?;
            break;
        }

        // Receive
        // Firstly, get the header
        let mut header = [0 as u8; 4];
        stream.read(&mut header).await?;
        let length = i32::from_be_bytes(header);

        // Secondly, read all data
        let mut read_length = 0;
        let mut all_bytes: Vec<u8> = vec![];
        loop {
            let mut buf = [0 as u8; 1024];
            let size = stream.read(&mut buf).await? as i32;
            for index in 0..size {
                all_bytes.push(buf[index as usize]);
            }
            read_length += size;
            if read_length >= length {
                break;
            }
        }
        let receive_text = String::from_utf8(all_bytes)?;
        println!("{}", receive_text);
    }

    match stream.shutdown().await {
        Ok(_) => {},
        Err(_) => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());
    let mut address = String::from("127.0.0.1:12345");
    if args.arg_args.len() > 0 {
        address = args.arg_args[0].clone();
    }

    match args.arg_command {
        Some(Command::Server) => {
            start_server(String::from(address)).await;
        },
        Some(Command::Client) => {
            match start_client(address).await {
                Ok(_) => {},
                Err(e) => {
                    println!("Failed To Start Client: {:?}", e);
                }
            }
        },
        None => {
            println!("Unknown Command");
        }
    }
}