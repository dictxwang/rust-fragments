use chrono::Local;
use tokio::sync::broadcast::{self, Sender};
use tokio::runtime;
use std::time::Duration;

// cargo run --bin about_tokio_channel
fn main() {

    let (sender, _): (Sender<i64>, _) = broadcast::channel(10);

    let sender_clone = sender.clone();
    let rt = runtime::Runtime::new().unwrap();
    rt.spawn(async move {
        for _ in 0..100 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            // 如果没有任何reveiver，send会发生异常
            let ts = Local::now().timestamp_micros();
            sender_clone.send(ts).unwrap();
            println!("Sended");
        }
    });

    let rt = runtime::Runtime::new().unwrap();
    rt.spawn(async move {
        let mut reveiver = sender.subscribe();
        loop {
            match reveiver.recv().await {
                Ok(ts) => {
                    let now = Local::now().timestamp_micros();
                    let duration = now - ts;
                    // duration about 80 micros
                    println!("Receive: {:?}, duration {:?} micros", ts, duration);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    });

    loop {

    }
}