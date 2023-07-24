#[derive(Clone, Copy)]
struct ParamA {
    pub id: i32
}

pub fn try_copy_param() {
    println!("\nthis is try_copy_param");
    let p: ParamA = ParamA{
        id: 123
    };
    let r: ParamA = change_param_value(p);
    println!("param id: {}", p.id);  // 如果结构体上没有加Copy特性，这里不允许再访问p的元素，因为已经发生了move

    // 方法前后的参数指针地址将不一样
    println!("before address: {:p}, after address: {:p}", &p, &r);
}

// 方法默认是move方式传参，如果参数类型增加了Copy特性，将变成Copy方式传参
fn change_param_value(mut p: ParamA) -> ParamA {
    p.id = 456;
    return p;
}