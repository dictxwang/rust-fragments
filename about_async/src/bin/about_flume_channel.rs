use std::time::Duration;

use anyhow::Result;
use chrono::Local;
use tokio::runtime;

// cargo run --bin about_flume_channel
#[tokio::main]
async fn main() -> Result<()> {

    let (sender, receiver) = flume::bounded(10);

    let rt = runtime::Runtime::new().unwrap();
    let sender_clone = sender.clone();
    rt.spawn(async move {
        for _ in 0..1 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            // 如果没有任何reveiver，send会发生异常
            let ts = Local::now().timestamp_micros();
            sender_clone.send(ts).unwrap();
            println!("Sended");
        }
    });

    // 支持多个生产者模式
    // let sender_clone = sender.clone();
    // rt.spawn(async move {
    //     for _ in 0..1 {
    //         // tokio::time::sleep(Duration::from_secs(1)).await;
    //         // 如果没有任何reveiver，send会发生异常
    //         let ts = Local::now().timestamp_micros();
    //         sender_clone.send(ts).unwrap();
    //         println!("Sended");
    //     }
    // });

    let rt = runtime::Runtime::new().unwrap();
    let reveiver_clone = receiver.clone();
    rt.spawn(async move {
        loop {
            match reveiver_clone.recv() {
                core::result::Result::Ok(ts) => {
                    let now = Local::now().timestamp_micros();
                    let duration = now - ts;
                    // duration about 80 micros
                    println!("Receive-1: {:?}, duration {:?} micros", ts, duration);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    });

    // 同一个消息只会被一个消费者接收到
    // let reveiver_clone = receiver.clone();
    // rt.spawn(async move {
    //     loop {
    //         match reveiver_clone.recv() {
    //             core::result::Result::Ok(ts) => {
    //                 let now = Local::now().timestamp_micros();
    //                 let duration = now - ts;
    //                 // duration about 80 micros
    //                 println!("Receive-2: {:?}, duration {:?} micros", ts, duration);
    //             },
    //             Err(e) => {
    //                 println!("Error: {:?}", e);
    //             }
    //         }
    //     }
    // });

    loop {

    }
}