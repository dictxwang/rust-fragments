use std::{collections::HashMap, fmt::Display};
use std::thread;
use std::time;

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
        println!("name={}, age={}", self.name, self.age);
    }

    pub fn grow_plus(&self) {

        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.spawn(async move {
            // let s = self;
            // thread::scope(|scope| {

            //     scope.spawn(|| async {
            //         self.print();
            //         // s.grow();
            //     });
            // });
            println!("hello");
        });
    }
}

// cargo run --bin about_closure_self
fn main () {

    let mut animal = Animal::new(String::from("cow"), 1);
    animal.grow();
    animal.add_info(String::from("weight"), String::from("10KG"));

    animal.print();

    animal.grow_plus();
    animal.print();
    thread::sleep(time::Duration::from_secs(1));
}