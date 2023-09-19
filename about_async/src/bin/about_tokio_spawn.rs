use anyhow::{Result, Ok};
use std::{thread, time};

// cargo run --bin about_tokio_spawn
#[tokio::main]
pub async fn main() -> Result<()> {

    let mut handlers = vec![];
    for i in 0..10 {
        handlers.push(tokio::spawn(async move {
            thread::sleep(time::Duration::from_secs(1));
            println!("this is task {i}")
        }));
    }
    
    for handler in handlers {
        handler.await.expect("something wrong");
    }

    println!("all task above finished");


    let rt = tokio::runtime::Runtime::new().unwrap();
    let handler = rt.spawn(async {
        println!("1234");
    });
    handler.await.expect("something wrong");

    Ok(())
}