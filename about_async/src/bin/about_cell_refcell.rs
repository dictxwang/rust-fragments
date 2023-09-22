use std::cell::Cell;
use std::cell::RefCell;
use anyhow::{Result, Ok};

#[derive(Debug)]
struct SCell {
    x: Cell<i32>,
    y: i32
}

async fn use_cell() {

    println!(">>>> this is use_cell");
    /*
        Cell只能包装Copy类型
    */
    let x = Cell::new(1);

    // y,x都是不可变引用，但是通过set方法均可以改变x的值
    let y = &x;
    let z = &x;
    y.set(3);
    z.set(4);
    println!("{:?}", x.get());  // 4

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(async move {
        let rx = &x;
        rx.set(1000);
    }).await.unwrap();
    rt.shutdown_background();
    // println!(":?", x.get());  // 经过了上面的move，这里不能再访问x了，因此不能用于多线程

    let mut b = Cell::new(10);
    let c = b.get_mut();
    // let d = b.get_mut();  // get_mut只可被借用一次
    *c = 5;
    println!("{:?}", b.get());  // 5


    let s = SCell{x: Cell::new(1), y: 2};
    let s1 = &s;
    let s2 = &s;
    s1.x.set(2);
    s2.x.set(3);
    println!("{:?}", s);  // x=3
}

#[derive(Debug)]
struct SRefCell {
    x: RefCell<i32>,
    y: i32
}

async fn use_refcell() {

    println!(">>>> this is use_refcell");

    /*
        RefCell可以包装任意类型
    */
    let x = RefCell::new(1);
    let y = &x;
    let z = &x;
    *x.borrow_mut() = 2;
    *y.borrow_mut() = 3;
    *z.borrow_mut() = 4;
    println!("{:?}", x.take());  // 4

    let s = SRefCell{x: RefCell::new(1), y: 1};
    let s1 = &s;
    let s2 = &s;
    *s1.x.borrow_mut() = 2;
    *s2.x.borrow_mut() = 3;
    println!("{:?}", s);  // x=3

}

// cargo run --bin about_cell_refcell
#[tokio::main]
async fn main() -> Result<()> {

    /*
        Cell和RefCell都只能在同一个线程中使用，不能用于跨线程场景
    */
    use_cell().await;
    use_refcell().await;

    Ok(())
}