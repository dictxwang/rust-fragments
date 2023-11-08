use std::collections::LinkedList;
use anyhow::{Result, Ok};
use std::sync::{Arc, Mutex};
use std::time;
use std::thread;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use async_once::AsyncOnce;
use rand::{self, Rng};
use tokio::runtime::Runtime;
use lazy_static::lazy_static;

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

    pub async fn async_new() -> Result<Self> {
        Ok(Self {
            messages: Arc::new(Mutex::new(LinkedList::new())),
            runtime: tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap()
        })
    }

    pub fn new_in_spawn() -> Self {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            Self {
                messages: Arc::new(Mutex::new(LinkedList::new())),
                runtime: tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap()
            }
        })
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
                            tokio::runtime::Runtime::new().unwrap().spawn(async move {
                                println!("inner thread, message={}", message);
                            });
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

    println!("use normal new");
    static CONSUMER: Lazy<Consumer> = Lazy::new(|| Consumer::new());

    // unsafe {  // 如果是 static mut consumer， 需要配合unsafe
        CONSUMER.start_auto_consume(4).await;

        let mut rand = rand::thread_rng();
        for i in 0..10 {
            thread::sleep(time::Duration::from_millis(rand.gen_range(100, 1000)));
            CONSUMER.push_message(format!("{}-{}", String::from("message-"), i)).await;
        }
    // }

    Ok(())
}

// 如果new方法是async类型，可以使用lazy_static!宏配合AsyncOnce调用
#[tokio::main]
async fn xmain() -> Result<()> {

    // println!("call async_new by lazy_static!...");
    // lazy_static! {
    //     static ref CONSUMER: AsyncOnce<Consumer> = AsyncOnce::new(async {
    //         let consumer = Consumer::async_new().await.unwrap();
    //         consumer
    //     });
    // }

    // CONSUMER.get().await.start_auto_consume(4).await;

    // let mut rand = rand::thread_rng();
    // for i in 0..100 {
    //     thread::sleep(time::Duration::from_millis(rand.gen_range(100, 1000)));
    //     CONSUMER.get().await.push_message(format!("{}-{}", String::from("message-"), i)).await;
    // }

    Ok(())
}


// 如果new方法是async类型，还可以使用OnceCell方式调用
#[tokio::main]
async fn ymain() -> Result<()> {

    println!("call async_new by OnceCell");
    static CONSUMER: OnceCell<Consumer> = OnceCell::new();
    let consumer = Consumer::async_new().await.unwrap();
    let _ = CONSUMER.set(consumer);

    CONSUMER.get().unwrap().start_auto_consume(4).await;

    let mut rand = rand::thread_rng();
    for i in 0..100 {
        thread::sleep(time::Duration::from_millis(rand.gen_range(100, 1000)));
        CONSUMER.get().unwrap().push_message(format!("{}-{}", String::from("message-"), i)).await;
    }

    Ok(())
}


// 注意：通过runtime.spawn这种方式，无论是用在new方法内部，还是用在调用new方法时，都是会报错误的
#[tokio::main]
async fn disabled_main() -> Result<()> {

    println!("use spawn new");
    static CONSUMER: Lazy<Consumer> = Lazy::new(|| Consumer::new_in_spawn());

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