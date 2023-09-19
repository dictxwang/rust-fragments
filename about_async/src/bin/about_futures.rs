use anyhow::{Result, Ok};
use rand::{self, Rng};
use std::{time, thread};

// cargo run --bin about_futures
#[tokio::main]
pub async fn main() -> Result<()> {

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(async {
        let mut tasks = vec![];
        for i in 0..10 {
            let task = tokio::task::spawn(async move {
                let mut rand = rand::thread_rng();
                let num = rand.gen_range(1, 3);
                thread::sleep(time::Duration::from_secs(num));
                format!("task-{i} hold on {num} seconds.")
            });
            tasks.push(task);
        }
    
        let results = futures::future::join_all(tasks).await;
        for res in results {
            if res.is_ok() {
                println!("{}", res.unwrap_or_default())
            } else {
                println!("something wrong")
            }
        }
    }).await.expect("something wrong");


    Ok(())
}