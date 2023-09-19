use std::{thread, time};

fn block_on() {

    let rt1 = tokio::runtime::Runtime::new().unwrap();
    let rt2 = tokio::runtime::Runtime::new().unwrap();

    // block_on会阻塞所有线程
    rt1.block_on(async {
        thread::sleep(time::Duration::from_secs(3));
        println!("[1] rt1: block_on");
    });

    // 会被rt1.block_on阻塞
    rt1.spawn(async{
        thread::sleep(time::Duration::from_secs(4));
        println!("[3] rt1: spawn");
    });

    // 会被rt1.block_on阻塞
    rt2.spawn(async {
        thread::sleep(time::Duration::from_secs(1));
        println!("[2] rt2: spawn");
    });

    thread::sleep(time::Duration::from_secs(6));
}

// cargo run --bin about_tokio_blockon
fn main() {

    // 不能在async中开启block_on，因此这里的main方法不能是async类型的
    block_on();
    println!("all functions finished.");
}