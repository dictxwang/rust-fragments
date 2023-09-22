use anyhow::{Result, Ok};
use std::{rc::{Rc, Weak}, cell::RefCell, borrow::BorrowMut};

async fn use_rc() {

    let a = Rc::new(1);
    println!("a's reference count is {}", Rc::strong_count(&a));  // 1

    let b = a.clone();
    println!("a's reference count is {}", Rc::strong_count(&a));  // 2

    let c = Rc::clone(&a);
    println!("a's reference count is {}", Rc::strong_count(&a));  // 3

    println!("{a}, {b}, {c}"); // 1, 1, 1

    drop(a);
    println!("c's reference count is {}", Rc::strong_count(&c));  // 2

    drop(b);
    println!("c's reference count is {}", Rc::strong_count(&c));  // 1
}

struct Car {
    name: String, 
    wheels: RefCell<Vec<Weak<Wheel>>>  // 使用Weak配置Rc使用，避免循环引用
}

struct Wheel {
    id: i32,
    car: Rc<Car>
}

async fn use_rc_weak() {
    let car = Rc::new(
        Car {
            name: "A".to_string(),
            wheels: RefCell::new(vec![])
        }
    );

    let wheel1 = Rc::new(
        Wheel {
            id: 1,
            car: Rc::clone(&car)
        }
    );
    let wheel2 = Rc::new(
        Wheel {
            id: 2,
            car: Rc::clone(&car)
        }
    );

    let mut wheels = car.wheels.borrow_mut();
    wheels.push(Rc::downgrade(&wheel1));  // 执行downgrade将是weak引用+1
    wheels.push(Rc::downgrade(&wheel2));

    drop(wheels);

    for wheel_weak in car.wheels.borrow().iter() {
        // 执行upgrade时，如果strong引用=0，将返回None，否则strong引用-1，并返回Some(Rc)
        let wheel = wheel_weak.upgrade().unwrap();
        println!("Wheel {} owned by {}", wheel.id, wheel.car.name);
    }
}

// cargo run --bin about_rc
#[tokio::main]
async fn main() -> Result<()> {

    /*
        std::rc下的Rc和Weak不能用于多线程环境
        多线程中可以使用std::sync下的Arc和Weak
    */
    use_rc().await;
    use_rc_weak().await;
    Ok(())
}