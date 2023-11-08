use std::{str::FromStr, ops::{Mul, Div}, collections::HashMap};

use primitive_types::U256;
use revm::primitives::{U256 as rU256};

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
}

#[test]
fn test04() {
    let u1 = rU256::from_str("0xedceff30864d0c59113a29113472807abd04c523799e1d9f56afb2bb1e3410d1").unwrap();
    println!("u1:{:?}", u1);
}

#[test]
fn test05() {
    let u1 = U256::from(123456789);
    let u2 = u1.mul(U256::from(75)).div(U256::from(100));
    println!("u2:{:?}", u2);
}

#[test]
fn test06() {
    let u1 = U256::from(10);
    let u2 = U256::from(9);
    let u3 = U256::from(10);
    let u4 = U256::from(11);
    println!("{:?}", u1.checked_sub(u2));  // Some(1)
    println!("{:?}", u1.checked_sub(u3));  // Some(0)
    println!("{:?}", u1.checked_sub(u4));  // None
    println!("{:?}", u1.checked_sub(u4).unwrap_or_default());  // 0

    println!("u1 == u3 is {:?}", u1 == u3);  // true
    println!("u1 eq u3 is {:?}", u1.eq(&u3));  // true
}

#[test]
fn test07() {

    let mut hm: HashMap<U256, String> = HashMap::new();
    hm.insert(U256::from(1), String::from("1"));
    hm.insert(U256::from(2), String::from("2"));
    hm.insert(U256::from(3), String::from("3"));
    hm.insert(U256::from(4), String::from("4"));

    let values: Vec<String> = hm.iter().filter(|(k,_)| **k > U256::from(2)).map(|(_, v)| v).cloned().collect();
    for v in values {
        println!("value {:?}", v);
    }
}

#[derive(Debug)]
struct SortItem {
    pub value: i32,
    pub name: String
}

#[test]
fn test08() {
    let mut vals = vec![];
    vals.push(10);
    vals.push(2);
    vals.push(4);
    vals.push(1);
    vals.sort();
    println!("{:?}", vals);  // [1, 2, 4, 10]

    let mut items = vec![];
    items.push(SortItem {value:2, name:String::from("o2")});
    items.push(SortItem {value:10, name:String::from("n10")});
    items.push(SortItem {value:4, name:String::from("n4")});
    items.push(SortItem {value:1, name:String::from("n1")});
    items.push(SortItem {value:2, name:String::from("n2")});
    items.push(SortItem {value:1, name:String::from("n11")});

    items.sort_by_key(|a| a.value);  // sort_by_key
    // items.sort_by(|a, b| 
    //     {if a.value < b.value {std::cmp::Ordering::Less} 
    //         else if a.value == b.value {std::cmp::Ordering::Equal} 
    //         else {std::cmp::Ordering::Greater}
    //     }
    // );  // or sort_by
    // for item in items {
    //     println!("{:?}", item);
    // }

    items.dedup_by_key(|a| a.value);
    for item in items {
        println!("{:?}", item);
    }

    // let names: Vec<String> = items.into_iter().map(|i|i.name).collect();
    // let joined = names.join(",");
    // println!("joined:{:?}", names.join("|"));

    // let uid = uuid::Uuid::new_v4();
    // let uid_str = format!("{:?}", uid);
    // println!("{}", uid_str);

    let mut nitems: Vec<SortItem> = vec![];
    nitems.push(SortItem { value: 1, name: String::from("a") });
    nitems.push(SortItem { value: 2, name: String::from("b") });
    nitems.push(SortItem { value: 3, name: String::from("a") });

}

#[test]
fn test09() {

    let mut map: HashMap<String, i32> = HashMap::new();
    let val: &i32 = map.get(&String::from("abc")).unwrap_or(&10);
    println!("value={:?}", val);
    println!("map is empty {:?}", map.is_empty());
    map.insert(String::from("xyz"), 12);
    println!("map is empty {:?}", map.is_empty());
}

// cargo test --release -- --nocapture -- --test test10
#[test]
fn test10() {

    let e = 1e18 as u32;
    println!("e is {:?}", e);
    let v = U256::from(e).checked_mul(U256::from(1)).unwrap();
    println!("v is {:?}", v);
    let u = U256::from(e);
    println!("u is {:?}", u);
}

// cargo test --release -- --nocapture -- --test test11
#[test]
fn test11() {
    //  frontrun_in 10973935802469135802467 weth_increase 1305063130332981971
    let rate = U256::from(10973935802469135802467u128).mul(U256::from(1e18 as u128)).div(U256::from(1305063130332981971u128));
    println!("rate: {:?}", rate);

}

// cargo test --release -- --nocapture -- --test test12
#[test]
fn test12() {
    let value = U256::from(100_000_000u128).checked_mul(U256::from(1e18 as u128)).unwrap_or_default();
    println!("value: {:?}", value);
}

// cargo test --release -- --nocapture -- --test test13
#[test]
fn test13() {
    let v = U256::from(1);
    let r = v.checked_sub(U256::from(2));
    println!("r is {:?}", r.ok_or_else(|| {
        println!("not ok");
    }));

    println!("xx is {:?}", v.checked_sub(U256::from(3)).unwrap_or_default());
}

enum Type {
    X,
    Y,
    Z,
}

// cargo test --release -- --nocapture -- --test test14
#[test]
fn test14() {

    let t1 = Type::Y;
    match t1 {
        Type::X => {
            println!("---abc");
        },
        Type::Y | Type::Z => {
            println!("123");
        },
    }
}