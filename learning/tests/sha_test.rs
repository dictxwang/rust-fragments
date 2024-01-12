use sha2::{Digest, Sha256};
use data_encoding::HEXUPPER;
use chrono::Local;

// cargo test --release -- --nocapture -- --test test_sha256
#[test]
pub fn test_sha256() {

    let update_id = 123456u64;

    let start = Local::now().timestamp_micros();
    let mut hasher = Sha256::new();
    hasher.update(update_id.to_be_bytes());
    // read hash digest and consume hasher
    let result = hasher.finalize();
    let hash = HEXUPPER.encode(&result);
    let cost = Local::now().timestamp_micros() - start;
    println!("cost:{:?}", cost);
    println!("{:?}", hash);
    println!("{:?}", hash[8..24].to_string());

    let u = 8596013685u64;
    let x = u % 2;
    println!("{:?}", x);
}