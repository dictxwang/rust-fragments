use std::{str::FromStr, ops::{Mul, Div}};

use primitive_types::U256;
use revm::primitives::{U256 as rU256, B160};

#[test]
fn test01() {
    println!("this is test");
    // 执行测试 cargo test -- --nocapture
    // -- --nocapture 是指测试过程中显示println的内容

    // -- 之前是编译的参数 -- 是test命令的参数
    // cargo test --release -- --nocapture -- --test-threads=1  # 单线程执行

    // cargo test --release -- --nocapture -- --test test01  # 指定运行单元测试
}

#[test]
fn test02() {
    // 字节序采用小端序存储
    let u1 = U256::from_dec_str("123456789101112131415161718").unwrap();
    println!("u1:{:?}", u1);  // 123456789101112131415161718
    println!("u1:{:?}", u1.0);  // [17479683217722162038, 6692605, 0, 0]
}

#[test]
fn test03() {
    let b1 = B160::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap();
    let b2 = B160::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap();
    assert_eq!(b1, b2);
}

#[test]
fn test04() {
    let u1 = rU256::from_str("0xedceff30864d0c59113a29113472807abd04c523799e1d9f56afb2bb1e3410d1").unwrap();
    println!("u1:{:?}", u1);

    let u2 = U256::from(u1);
    println!("u2:{:?}", u2);
}

#[test]
fn test05() {
    let u1 = U256::from(123456789);
    let u2 = u1.mul(U256::from(75)).div(U256::from(100));
    println!("u2:{:?}", u2);
}