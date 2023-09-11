use tokio::time;

async fn f1() -> i32 {
    time::sleep(time::Duration::from_secs(3)).await;
    println!("this is f1");
    return 3;
}

async fn f2() -> i32 {
    time::sleep(time::Duration::from_secs(5)).await;
    println!("this is f2");
    return 5;
}

fn test_join() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    let start = time::Instant::now();
    rt.block_on(async {
        tokio::join!(f1(), f2());
    });
    let end = time::Instant::now();
    let cost = end.duration_since(start).as_millis();
    println!("join!: cost time {}ms", cost);
}

fn test_select() {

    let rt = tokio::runtime::Runtime::new().unwrap();
    let start = time::Instant::now();
    rt.block_on(async {
        let res = tokio::select! {
            v1 = f1() => v1,
            v2 = f2() => v2,
        };
        println!("select! result: {}", res);
    });
    let end = time::Instant::now();
    let cost = end.duration_since(start).as_millis();
    println!("select!: cost time {}ms", cost);
}

fn main() {
    test_join();
    test_select();
}