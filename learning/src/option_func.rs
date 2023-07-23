pub fn get_option_value(flag: i32) -> Option<String> {

    match flag {
        0 => return Some(format!("value is {}", "Zero")),
        1 => return Some(format!("value is {}", "One")),
        _ => return Some(format!("value is {}", "Unknown"))
    }
}

pub fn foo() -> Result<i32, String> {
    return Err(String::from("Error Occur"));
}

pub fn bar() -> Option<i32> {
    foo().ok()?;  // 使用?简化对调用结果的判断，如果是Err会结束后面的流程
    println!("below foo().ok()");
    return Some(0);
}
