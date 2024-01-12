use std::sync::mpsc;
use std::time;
use std::thread;

use chrono::Local;

fn single_sender_and_single_reveiver() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {

        tx.send("hello, world").unwrap();
    });

    // recv会阻塞当前线程
    let msg = rx.recv().unwrap();
    println!("single message: {}", msg);
}

fn single_sender_and_single_reveiver_for() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("you"),
            String::from("are"),
            String::from("my"),
            String::from("firend"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Got message: {}", msg);
    }
}

fn sync_channel() {
    let(tx, rx) = mpsc::sync_channel(0);

    thread::spawn(move || {
        println!("[1] before send");
        tx.send("message: hello").unwrap();
        println!("[5] after send");
    });

    println!("[2] before sleep");
    thread::sleep(time::Duration::from_secs(3));
    println!("[3] after sleep");

    // rx.recv()前，tx.send()会被阻塞
    println!("[4] reveive: {}", rx.recv().unwrap());
}

fn channel_delay() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for _ in 0..100 {
            // thread::sleep(time::Duration::from_secs(1));
            let ts = Local::now().timestamp_micros();
            tx.send(ts).unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            let ts = rx.recv().unwrap();
            let now = Local::now().timestamp_micros();
            let duration = now - ts;
            println!("Receive: {:?}, duration {:?} micros", ts, duration);
        }
    });
}

fn sync_channel_delay() {
    let (tx, rx) = mpsc::sync_channel(0);

    let tx_clone = tx.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            // thread::sleep(time::Duration::from_secs(1));
            let ts = Local::now().timestamp_micros();
            tx_clone.send(ts).unwrap();
        }
    });

    let tx_clone = tx.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            // thread::sleep(time::Duration::from_secs(1));
            let ts = Local::now().timestamp_micros();
            tx_clone.send(ts).unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            let ts = rx.recv().unwrap();
            let now = Local::now().timestamp_micros();
            let duration = now - ts;
            println!("Receive: {:?}, duration {:?} micros", ts, duration);
        }
    });
}

// cargo run --bin about_channel
fn main() {

    // single_sender_and_single_reveiver();
    // single_sender_and_single_reveiver_for();
    // sync_channel();
    sync_channel_delay();
    // channel_delay();

    loop {

    }
}