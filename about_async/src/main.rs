use tokio::{time};

async fn async_print() -> i32 {
    time::sleep(time::Duration::from_secs(3)).await;
    println!("[2] in async_print after time::sleep");
    return 1;
}

async fn async_print_again() -> i32 {
    time::sleep(time::Duration::from_secs(2)).await;
    println!("[4] in async_print_again after time::sleep");
    return 1;
}

#[tokio::main]
async fn main() {
    let feture = async_print();
    let feture_again = async_print_again();
    println!("[1] Hello, world!");
    println!("[3] feture:{}", feture.await);
    println!("[5] feture:{}", feture_again.await);
}
