pub fn closure_first() {
    println!("\nthis is closure_first");
    
    let add = |i| i + 1;
    println!("2+1={}", add(2));

    let one = || 1;  // 无参的方法
    println!("get one {}", one());
}

pub fn closure_second() {
    println!("\nthis is closure_second");
    let color = String::from("white");
    let paint_white = || {
        println!("the color is {}", color);
        println!("paint the wall is {}", color);
    };

    paint_white();
}

pub fn closure_third() {
    println!("\nthis is clouse_third");
    // 高阶函数（High Order Function）
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n|n*n)
        .take_while(|&n|n<1000)  // 循环条件
        .filter(|n|n%2==1)  // 过滤其中的奇数
        .fold(0, |sum, i|sum + i);  // 将过滤结果叠加
    println!("sum of squared odd numbers below 1000: {}", sum_of_squared_odd_numbers);
}

#[allow(dead_code)]
pub fn unused_function() {}