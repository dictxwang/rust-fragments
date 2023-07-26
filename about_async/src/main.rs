use tokio::{time};

async fn async_print() -> i32 {
    time::sleep(time::Duration::from_secs(3)).await;
    println!("[2] in async_print after time::sleep");
    return 1;
}

#[tokio::main]
async fn main() {
    let feture = async_print();
    println!("[1] Hello, world!");
    println!("[3] feture:{}", feture.await);
}
