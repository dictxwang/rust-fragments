use anyhow::Result;
use std::time;
use std::thread;

// cargo run --bin about_tokio_task
#[tokio::main]
async fn main() -> Result<()> {

    spawn_blocking();
    spawn().await;
    block_in_place();
    Ok(())
}

async fn spawn() {

    println!("[function spawn]");
    // 依据cpu核数并行
    let mut handlers = vec![];
    for i in 0..5 {
        handlers.push(tokio::task::spawn(async move {
            thread::sleep(time::Duration::from_secs(2));
            
            println!("spawn loop index is {}", i);
        }));
    }

    for handler in handlers {
        handler.await.unwrap();
    }
}

fn spawn_blocking() {

    println!("[function spawn_blocking]");
    // 并行
    for i in 0..5 {
        tokio::task::spawn_blocking(move || {
            thread::sleep(time::Duration::from_secs(1));
            println!("spawn_blocking loop index is {}", i);
        });
    }
}

fn block_in_place() {

    println!("[function block_in_place]");
    // 顺序执行
    for i in 0..5 {
        tokio::task::block_in_place(move || {
            thread::sleep(time::Duration::from_secs(1));
            println!("block_in_place loop index is {}", i);
        });
    }
    // 这里不会阻塞后续的代码
}