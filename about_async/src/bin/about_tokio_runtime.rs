use std::{thread, time};

fn test01() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(async {
        thread::sleep(time::Duration::from_secs(3));
        // tokio::time::sleep(time::Duration::from_secs(5)).await;
        println!("spawn 01");
    });
    
    rt.spawn(async {
        thread::sleep(time::Duration::from_secs(3));
        // tokio::time::sleep(time::Duration::from_secs(5)).await;
        println!("spawn 02");
    });

    thread::sleep(time::Duration::from_secs(6));
}

fn test02() {

    let rt1 = tokio::runtime::Runtime::new().unwrap();
    let rt2 = tokio::runtime::Runtime::new().unwrap();
    // spawn_blocking会阻塞同一个runtime的线程
    rt1.spawn_blocking(|| {
        thread::sleep(time::Duration::from_secs(5));
        println!("[3] rt1: spawn_blocking");
    });

    // 会被rt1.spawn_blocking阻塞
    rt1.spawn(async{
        thread::sleep(time::Duration::from_secs(3));
        println!("[2] rt1: spawn");
    });

    // 不会被rt1.spawn_blocking阻塞
    rt2.spawn(async {
        thread::sleep(time::Duration::from_secs(1));
        println!("[1] rt2: spawn");
    });
    thread::sleep(time::Duration::from_secs(6));
}

fn test03() {

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

async fn sleep_println(duration: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;
    println!("sleep and println");
}

fn test04() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn({
        sleep_println(3)
    });
    
    rt.spawn({
        sleep_println(4)
    });

    thread::sleep(time::Duration::from_secs(5));
}

fn main() {
    test01();
    test02();
    test03();
    test04();
}