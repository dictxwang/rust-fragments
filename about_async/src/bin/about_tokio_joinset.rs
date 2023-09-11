use std::{thread, time};
use anyhow::Result;
use tokio::task::JoinSet;
use rand::Rng;

async fn get_num(times: u64) -> u64 {

    tokio::time::sleep(time::Duration::from_secs(1)).await;
    times
}

async fn proc_num(num: u64) -> String {

    let mut reg = rand::thread_rng();
    thread::sleep(time::Duration::from_secs(reg.gen_range(2, 10)));
    println!("num: {}", num);
    String::from("OK")
}

async fn test05() -> Result<JoinSet<()>> {

    let mut set = JoinSet::new();
    set.spawn(async move {
        let mut times = 0;
        let rt = tokio::runtime::Runtime::new().unwrap();
        loop {
            times += 1;
            let n = get_num(times).await;
            rt.spawn(async move {
                let str = proc_num(n).await;
                println!("after proc_num: {}", str);
            });

            if times > 10 {
                break;
            }
        }
    });

    set.spawn(async move {
        let mut times = 10;
        let rt = tokio::runtime::Runtime::new().unwrap();
        loop {
            times += 1;
            let n = get_num(times).await;
            rt.spawn(async move {
                let str = proc_num(n).await;
                println!("after proc_num: {}", str);
            });

            if times > 20 {
                break;
            }
        }
    });

    // set.spawn(async {
    //     let mut times = 0;
    //     loop {
    //         times += 1;
    //         tokio::time::sleep(time::Duration::from_secs(5)).await;
    //         println!("after sleep");

    //         if times > 10 {
    //             break;
    //         }
    //     }
    // });

    Ok(set)
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(mut set) = test05().await {
        // while let Some(res) = set.join_next().await {
        //     println!("res: {:?}", res)
        // }
        set.join_next().await;
    }

    println!("next");
    tokio::time::sleep(time::Duration::from_secs(20)).await;
    Ok(())
}