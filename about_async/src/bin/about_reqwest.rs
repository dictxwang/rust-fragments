use std::{sync::Arc, time::Duration};

use chrono::Local;
use reqwest::Result;

fn test_01() -> Result<()> {
    let url = "https://yt3.ggpht.com/ZC9IMkqrw-OuYVM5zU5-shKEK7DfCoym6Dl5HPcwh4pj_u5gdkJXxggDa91GZiDgwcbmPjgw4S0=s88-c-k-c0x00ffffff-no-rj";
    let start_time = Local::now().timestamp_micros();
    reqwest::blocking::get(url)?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_01 cost: {}", cost);

    let start_time = Local::now().timestamp_micros();
    reqwest::blocking::get(url)?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_01 cost: {}", cost);

    let start_time = Local::now().timestamp_micros();
    reqwest::blocking::get(url)?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_01 cost: {}", cost);

    let start_time = Local::now().timestamp_micros();
    reqwest::blocking::get(url)?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_01 cost: {}", cost);

    Ok(())
}

async fn test_02() -> Result<()> {

    let url = "https://k.sinaimg.cn/n/sinacn/w564h1001/20180112/3290-fyqnick9282717.jpg/w700d1q75cms.jpg?by=cms_fixed_width";
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(5usize)
        .pool_idle_timeout(Duration::from_micros(100000))
        .tcp_keepalive(Duration::from_micros(100000))
        .danger_accept_invalid_certs(true)
        
        .build()
        .unwrap();

    let start_time = Local::now().timestamp_micros();
    client.get(url).send().await?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_02 cost: {}", cost);

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let start_time = Local::now().timestamp_micros();
    let get = client.get(url);
    get.send().await?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_02 cost: {}", cost);

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let start_time = Local::now().timestamp_micros();
    client.get(url).send().await?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_02 cost: {}", cost);

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let start_time = Local::now().timestamp_micros();
    client.get(url).send().await?;
    let cost = Local::now().timestamp_micros() - start_time;
    println!("test_02 cost: {}", cost);

    Ok(())
}

async fn test_03() -> Result<()> {

    let url = "https://k.sinaimg.cn/n/sinacn/w564h1001/20180112/3290-fyqnick9282717.jpg/w700d1q75cms.jpg?by=cms_fixed_width";
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(5usize)
        .pool_idle_timeout(Duration::from_micros(10000000))
        // .tcp_keepalive(Duration::from_micros(100000))
        .danger_accept_invalid_certs(true)
        
        .build()
        .unwrap();


    let client_arc = Arc::new(client);

    let client = client_arc.clone();
    let start_time = Local::now().timestamp_micros();
    tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cost = Local::now().timestamp_micros() - start_time;
        println!("test_02_01 cost: {}", cost);
    });

    let client = client_arc.clone();
    tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cost = Local::now().timestamp_micros() - start_time;
        println!("test_02_02 cost: {}", cost);
    });

    let client = client_arc.clone();
    tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cost = Local::now().timestamp_micros() - start_time;
        println!("test_02_03 cost: {}", cost);
    });

    tokio::time::sleep(std::time::Duration::from_micros(5)).await;

    let client = client_arc.clone();
    let start_time = Local::now().timestamp_micros();
    tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cost = Local::now().timestamp_micros() - start_time;
        println!("test_02_04 cost: {}", cost);
    });

    let client = client_arc.clone();
    tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cost = Local::now().timestamp_micros() - start_time;
        println!("test_02_05 cost: {}", cost);
    });

    let client = client_arc.clone();
    tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(_) => {},
            Err(_) => {},
        }
        let cost = Local::now().timestamp_micros() - start_time;
        println!("test_02_06 cost: {}", cost);
    });
    tokio::time::sleep(std::time::Duration::from_secs(8)).await;

    Ok(())
}

// cargo run --bin about_reqwest
#[tokio::main]
async fn main() -> Result<()>{
    // test_02().await?;
    test_03().await?;
    Ok(())
}

// fn main() -> Result<()>{
//     test_01()?;
//     Ok(())
// }