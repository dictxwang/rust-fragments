use std::{thread, time};

fn main() {
    let handle = thread::spawn(|| {
        let t = time::Duration::from_secs(3);
        thread::sleep(t);
        // let now = time::Duration::now();
        println!("in thread spawn");
    });
    handle.join().unwrap();

    let text = "我爱北京天安门";
    // move将text的所有权转移到线程方法，避免提前释放
    let handle2 = thread::spawn(move || {
        println!("{}", text);
    });
    handle2.join().unwrap();

}