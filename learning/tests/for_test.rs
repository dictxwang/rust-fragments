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

    println!("Uuid: {}", format!("{:?}", uuid::Uuid::new_v4()));
    println!("Uuid: {}", format!("{:?}", uuid::Uuid::new_v4()));
    println!("Uuid: {}", format!("{:?}", uuid::Uuid::new_v4()));
    println!("Uuid: {}", format!("{:?}", uuid::Uuid::new_v4()));

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

// cargo test --release -- --nocapture -- --test test15
#[test]
fn test15() {
    
    let mut s1 = String::new();
    s1.insert_str(0, "123");
    let mut s2 = String::new();
    s2.insert_str(0, "123");
    let eq = s1.eq(&s2);
    println!("s1 equals s2 is {:?}", eq);  // true
    let eq = s1 == s2;
    println!("s1 equals s2 is {:?}", eq);  // true

    let s3 = "123";
    let s4 = "123";
    let eq = s3.eq(s4);
    println!("s3 equals s4 is {:?}", eq);  // true
    let eq = s3 == s4;
    println!("s3 equals s4 is {:?}", eq);  // true

    let s5 = String::from("123");
    let s6 = String::from("123");
    let eq = s5.eq(&s6);
    println!("s5 equals s6 is {:?}", eq);  // true
    let eq = s5 == s6;
    println!("s5 equals s6 is {:?}", eq);  // true
}


// cargo test --release -- --nocapture -- --test test16
#[test]
fn test16() {

    // revenue=16427034431455232 fgas=135713 bgas=101151 bfee=49473784859 mfee=99839830222 profit=0~4708475854613056 no_dust=1

    let fgas = U256::from(178111 as u64);
    let bgas = U256::from(124172 as u64);
    let bfee = U256::from(18409669273 as u128);
    let revenue = U256::from(5633655116398592 as u128);
    let dust = U256::from(386100000000000u128);
    let no_dust = 2u64;

    let revenue_minus = revenue.checked_sub(fgas * bfee).unwrap_or_default();
    let bribe = revenue_minus + dust * no_dust;
    let max_fee = bribe / bgas;
    println!("{:?}", max_fee);

}

// cargo test --release -- --nocapture -- --test test17
#[test]
fn test17() {

    let a = 1f64;
    let b = 3f64;
    println!("{}", a/b);


    let vs = vec![1.2f64, 2.8f64, -0.3f64];
    let sum_of_vec = |vs: Vec<f64>| {
        vs.into_iter().reduce(|t, c| t+c).unwrap()
    };
    // let sum = vs.into_iter().reduce(|t,c| t+c).unwrap();
    println!("sum is {:?}", sum_of_vec(vs.clone()));
    println!("{:?}", vs);

    let x = 0.0f64;
    let y = 12.34f64;
    println!("x*y={:?}", x * y == 0.0f64);

    let z: f64 = 1.2345678;
    let p1 = 10i32.pow(3);
    println!("{}", z * p1 as f64);
    println!("{}", (z * p1 as f64) as i32);
    println!("z is {}", ((z * p1 as f64) as i32) as f64 / p1 as f64);
}


// cargo test --release -- --nocapture -- --test test18
#[test]
fn test18() {

    let quote_asset_amount = 2f64;
    let symbol_price = 0.02183f64;
    let step_size = 1.0f64;
    let quote_precision = 8u32;

    let lot = quote_asset_amount / symbol_price;
    let lot = (lot / step_size) as i32;
    let lot = lot as f64 * step_size;

    println!(">>> lot:{:?}", lot);
    
    let pow = 10i64.pow(quote_precision);  // 用i64防止溢出
    let final_lot = ((lot * pow as f64) as i64) as f64 / pow as f64;

    println!(">>> final_lot:{:?}", final_lot);
}