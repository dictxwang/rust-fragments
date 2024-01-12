use anyhow::{Result, Ok};
use tokio::runtime::Runtime;

async fn div(a: i32, b: i32) -> Result<i32> {

    println!("in div");
    Ok(a / b)
    // Err(anyhow!("Failed"))
}

// cargo run --bin about_tokio_runtime_nested
#[tokio::main]
async fn main() -> Result<()> {

    let rt = Runtime::new().unwrap();
    let rt_inner = Runtime::new().unwrap();

    rt.spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            println!("do div");
            rt_inner.spawn(div(1, 1));
        }
    });

    loop {

    }
}