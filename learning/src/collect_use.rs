use std::collections::HashMap;
use std::collections::HashSet;

pub fn use_map() {
    println!("\nthis is use_map");
    let mut contacts: HashMap<String, String> = HashMap::new();
    contacts.insert(String::from("zhao"), String::from("123456"));
    contacts.insert(String::from("qian"), String::from("234567"));

    for key in contacts.keys() {
        let value = contacts.get(key);
        println!("{}={}", key, value.unwrap());
    }

    for (_, value) in contacts.iter_mut() {
        let val = format!("{}{}", value, value);
        *value = val;  // 遍历过程中，更新value
    }

    for (key, value) in contacts.iter() {
        println!("{}={}", key, value);
    }
}

pub fn use_set() {
    println!("\nthis is use_set");
    let v = vec![1,1,2,3,4,5,3];
    let set: HashSet<i32> = v.into_iter().collect();
    for val in set {
        println!("{}", val)
    }
}