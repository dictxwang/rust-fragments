pub fn process_for() {
    println!("\nthis is process_for");
    // 循环输出1~3
    for i in 1..4 {
        println!("{}", i);
    }

    // 循环输出1~3
    for i in 1..=3 {
        println!("{}", i);
    }
}

pub fn process_array() {
    println!("\nthis is process_array");
    let ax: [i32; 3] = [1,2,3];
    // 让出数组所有权给ay
    let ay = ax;
    for item in ay {
        println!("{}", item);
    }

    // 赋相同的值0
    let mut az: [i32; 5] = [0; 5];
    for index in 0..az.len() {
        az[index] = (index * 2) as i32;
    }
    for item in az.iter() {
        println!("{}", item);
    }
}

pub fn process_loop() {
    println!("\nthis is process_loop");
    let mut count: i32 = 0;
    loop {
        println!("count={}", count);
        count += 1;
        if count >= 3 {
            break;  
        }
    }
}

pub fn process_while() {
    println!("\nthis is process_while");
    let mut count = 1;
    while count <= 3 {
        println!("count={}", count);
        count += 1;
    }
}

pub fn process_match(val: i32, boolean: bool) {
    println!("\nthis is process_math");
    match val % 2 {
        0 => println!("this is even"),
        1 => println!("this is odd"),
        _ => println!("this is default")
    }

    let res: i32 = match boolean {
        false => 0,
        true => 1
    };
    println!("result is {}:{}", boolean, res);

    let pair = (val, boolean);
    match pair {
        (x, y) if x > 0 && y => println!("success"),  // 这里的if判断是卫语句
        _ => println!("failure")
    }
}

#[test]
fn test01() {
    println!("this is test");
    // 执行测试 cargo test -- --nocapture
    // -- --nocapture 是指测试过程中显示println的内容
}