use std::collections::HashMap;

pub struct Person {
    name: String,
    friends: HashMap<String, String>,
}

impl Person {
    fn new(name: String) -> Self {
        Self {
            name: name,
            friends: HashMap::new(),
        }
    }

    pub fn say_hi(&self) {
        println!("Hi, this is {}", self.name);
    }

    pub fn add_friend(&mut self, name: String, address: String) {
        self.friends.insert(name, address);
    }

    pub fn get_friend_info(&self, name: String) -> Option<&String> {
        if self.friends.contains_key(&name) {
            self.friends.get(&name)
        } else {
            None
        }
    }
}

// cargo run --bin about_struct_use
fn main() {

    let mut person = Person::new(String::from("liu"));
    person.say_hi();

    person.add_friend(String::from("zhao si"), String::from("深圳"));
    let friend_info = person.get_friend_info(String::from("zhao si")).unwrap();
    println!("his friend is from {}", friend_info);
}