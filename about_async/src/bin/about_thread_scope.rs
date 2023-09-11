use std::thread;
use std::time;
use tokio::task::JoinSet;

fn first() {

    let first = 1;
    let second = 2;
    thread::spawn(move || {

        thread::scope(|scope| {
            scope.spawn(|| {
                loop{
                    thread::sleep(time::Duration::from_secs(1));
                    println!("first value is {}", first);
                }
            });
            scope.spawn(|| {
                loop {
                    thread::sleep(time::Duration::from_secs(2));
                    println!("second value is {}", second);
                }
            });

            println!("this is scope inner");
        });
    });

    println!("this is spawn outer");
    // thread::sleep(time::Duration::from_secs(20));
}

fn second() -> JoinSet<()> {
    let mut set = JoinSet::new();
    set.spawn(async move {
        loop {
            thread::sleep(time::Duration::from_secs(3));
            println!("this is set spawn");
        }
    });

    set
}

#[tokio::main]
async fn main() {

    first();
    let mut set = second();
    set.join_next().await;

    println!("this is main....");
    loop {
        
    }
}