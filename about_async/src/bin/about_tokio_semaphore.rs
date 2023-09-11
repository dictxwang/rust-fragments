use std::sync::Arc;
use tokio::sync::Semaphore;
use std::thread;
use std::time;
use anyhow::Result;

// cargo run --bin about_tokio_semaphore
#[tokio::main]
async fn main() -> Result<()> {

    let semaphore = Arc::new(Semaphore::new(3));
    let mut handlers = vec![];

    for i in 0..7 {

        let semaphore_clone = semaphore.clone();
        handlers.push(tokio::spawn(async move {
            let permit = semaphore_clone.acquire_owned().await.unwrap();
            println!("this is {}", i);
            thread::sleep(time::Duration::from_secs(2));
            drop(permit);  // 可以主动释放，否则当方法结束会自动释放
        }));
    }

    let start = time::Instant::now();
    for handler in handlers {
        handler.await.unwrap();
    }
    let end = time::Instant::now();
    let cost = end.duration_since(start).as_millis();
    println!("total cost: {}ms", cost);

    Ok(())
}