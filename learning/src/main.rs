mod solid;
use learning::prelude::biology::BiologyAction;
// 在main中引入module依赖（这里直接找到solid.rs文件）
use solid::*;
mod process;
mod closure;
mod biology_func;
mod option_func;
mod collect_use;
mod clone_copy;
mod about_tokio;
mod for_test;

use learning::prelude::circle::*;
use learning::prelude::shape::*;

use learning::prelude::animal::*;
use learning::prelude::mammal::*;

use crate::closure::closure_third;  // 需要同时引入trait的定义

// 在main中定义mod
mod mod_main {
    pub fn minus(a: i32, b: i32) -> i32 {
        return a - b;
    }
}

// 在main中定义函数
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn add_mius(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a - b);
}

fn main() {
    println!("Hello, world!");

    // 定义不可变参数
    let iv1 = 1;
    // 定义可变参数
    let mut iv2: i32 = 2;
    println!("iv1={}, iv2={}", iv1, iv2);  // iv1=1, iv2=2
    iv2 += 10;
    println!("iv1={}, iv2={}", iv1, iv2);  // iv1=1, iv2=12

    println!("iv1+iv2={}", add(iv1, iv2)); // iv1+iv2=13
    // module的方法通过::调用
    println!("iv2-iv1={}", mod_main::minus(iv2, iv1));  // iv2-iv1=11

    // 使用元组接收多个返回值
    let res = add_mius(iv1, iv2);
    println!("iv1,iv2 add minus ({}, {})", res.0, res.1);

    // 解构元组
    let (res_add, res_minus) = res;
    println!("add minus ({}, {})", res_add, res_minus);

    let cube = Cube::new(10.5, 20.0, 5.0);
    // the cube's volume is 1050
    println!("the cube's volume is {}", cube.volume());

    let c01: Circle = Circle::new(5.0);
    println!("the circle's area is {}", c01.area());

    // 泛型，特性继承
    let dog01 : Dog<String> = Dog::new(String::from("Jack"), String::from("white"));
    dog01.show_name();
    dog01.do_jump();

    // 泛型方法，及多重约束
    // biology_func::biology_show_name(dog01);

    // process for
    process::process_for();
    // process array
    process::process_array();

    // process loop
    process::process_loop();
    // process while
    process::process_while();

    // process match
    process::process_match(23, false);

    // 闭包
    closure::closure_first();
    closure::closure_second();
    closure_third();

    // option使用
    let opt_val = option_func::get_option_value(1);
    println!("{}", opt_val.unwrap());

    println!("option::bar result is {:?}", option_func::bar());

    // collection使用
    collect_use::use_map();
    collect_use::use_set();

    // copy 与 clone
    clone_copy::try_copy_param();

    // tokio库的应用
    about_tokio::tokio_first();
    about_tokio::tokio_second();
    about_tokio::tokio_third();
}
