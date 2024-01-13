use chrono::Local;
use tokio::sync::mpsc;
use tokio::runtime;
use std::time::Duration;

// cargo run --bin about_tokio_channel_mpsc
fn main() {

    let (sender, mut receiver) = mpsc::channel(1);

    let rt = runtime::Runtime::new().unwrap();
    let sender_clone = sender.clone();
    rt.spawn(async move {
        for _ in 0..5 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            // 如果没有任何reveiver，send会发生异常
            let ts = Local::now().timestamp_micros();
            sender_clone.send(ts).await.unwrap();
            println!("Sender-1 Sended");
        }
    });

    let sender_clone = sender.clone();
    rt.spawn(async move {
        for _ in 0..5 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            // 如果没有任何reveiver，send会发生异常
            let ts = Local::now().timestamp_micros();
            sender_clone.send(ts).await.unwrap();
            println!("Sender-2 Sended");
        }
    });

    let rt = runtime::Runtime::new().unwrap();
    rt.spawn(async move {
        loop {
            match receiver.recv().await {
                Some(ts) => {
                    let now = Local::now().timestamp_micros();
                    let duration = now - ts;
                    // duration about 80 micros
                    println!("Receive: {:?}, duration {:?} micros", ts, duration);
                },
                None => {
                }
            }
        }
    });

    loop {

    }
}