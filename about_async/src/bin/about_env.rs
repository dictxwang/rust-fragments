use dotenv::dotenv;
use std::env;

// cargo run --bin about_env
fn main() {
    println!("Hello, this is about env");
    dotenv().ok();

    // 打印系统的环境变量
    for (k, v) in env::vars() {
        println!("{}={}", k, v);
    }

    // 打印自定义的环境变量（即.env中定义的变量）
    println!("DB_URL={}", env::var("DB_URL").unwrap());
    println!("LOG_LEVEL={}", env::var("LOG_LEVEL").unwrap());

    let a: u64 = 1231311;
    println!("=={}", a);
}