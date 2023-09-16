use std::collections::LinkedList;
use anyhow::{Result, Ok};
use std::sync::{Arc, Mutex};
use std::time;
use std::thread;
use once_cell::sync::Lazy;
use rand::{self, Rng};
use tokio::runtime::Runtime;

pub struct Consumer {
    messages: Arc<Mutex<LinkedList<String>>>,
    runtime: Runtime
}

impl Consumer {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(LinkedList::new())),
            runtime: tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap()
        }
    }

    pub async fn push_message(&self, msg: String) {
        let mut locked_list = self.messages.lock().unwrap();
        locked_list.push_back(msg);
        drop(locked_list);
    }

    async fn pop_message(&self) -> Option<String> {
        let mut locked_list = self.messages.lock().unwrap();
        if locked_list.len() > 0 {
            locked_list.pop_front()
        } else {
            None
        }
    }

    pub async fn start_auto_consume(&'static self, thread_num: i32) {

        for index in 0..thread_num {
            self.runtime.spawn(async move {
                loop {
                    match self.pop_message().await {
                        Some(message) => {
                            println!("{} consume message: {}", index, message);
                            let mut rand = rand::thread_rng();
                            thread::sleep(time::Duration::from_millis(rand.gen_range(500, 800)))
                        },
                        None => {
                            thread::sleep(time::Duration::from_secs(1));
                        }
                    }
                }
            });
        }
        println!("start consume...");
    }
}

// cargo run --bin about_auto_consume
#[tokio::main]
async fn main() -> Result<()> {

    static CONSUMER: Lazy<Consumer> = Lazy::new(|| Consumer::new());

    // unsafe {  // 如果是 static mut consumer， 需要配合unsafe
        CONSUMER.start_auto_consume(4).await;

        let mut rand = rand::thread_rng();
        for i in 0..100 {
            thread::sleep(time::Duration::from_millis(rand.gen_range(100, 1000)));
            CONSUMER.push_message(format!("{}-{}", String::from("message-"), i)).await;
        }
    // }

    Ok(())
}