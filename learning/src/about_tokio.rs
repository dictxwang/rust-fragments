use tokio::{time, runtime::Runtime};

pub fn tokio_first() {
    println!("\nthis is tokio_first");
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        time::sleep(time::Duration::from_secs(2)).await;
        println!("in block_on method");
    });
}

async fn delay_print() {
    time::sleep(time::Duration::from_secs(3)).await;
    println!("in delay print");
}

pub fn tokio_second() {
    let rt: Runtime = Runtime::new().unwrap();
    rt.block_on(async {
        delay_print().await;
    });
}

async fn delay_return() -> i32 {
    time::sleep(time::Duration::from_secs(2)).await;
    return 123;
}

pub fn tokio_third() {
    let rt: Runtime = Runtime::new().unwrap();
    rt.block_on(async {
        let res = delay_return().await;
        println!("in tokio_third, result is {}", res);
    });
}