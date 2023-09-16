use std::{collections::HashMap, fmt::Display};
use std::thread;
use std::time;
use once_cell::sync::Lazy;

pub struct Animal<T> {
    name: T,
    age: i32,
    info: HashMap<String, String>,
}

impl <T> Animal<T> where T: Display + Send + Sync + 'static {
    fn new(name: T, age: i32) -> Self {
        Self {
            name: name,
            age: age,
            info: HashMap::new(),
        }
    }

    pub fn grow(&mut self) {
        self.age = self.age + 1;
    }

    pub fn add_info(&mut self, key: String, value: String) {
        self.info.insert(key, value);
    }

    pub fn print(&self) {
        println!("name={}, age={}, info_count={}", self.name, self.age, self.info.len());
    }

    pub fn static_grow(&'static mut self) {

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.spawn(async {
            loop {
                thread::sleep(time::Duration::from_secs(1));
                self.grow();
                self.add_info(String::from("color"), String::from("grey"));
                self.print();
            }
        });
    }

}

// cargo run --bin about_closure_static
fn main () {

    static mut animal: Lazy<Animal<String>> = Lazy::new(|| Animal::new(String::from("sheep"), 1));
    unsafe {
        animal.static_grow();
    }

    println!("step 002");
    loop {
        
    }
}