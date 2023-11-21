use std::cell::RefCell;
use std::sync::Barrier;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::sync::Once;
use std::thread;
use std::time;
use rand::{self, Rng};

fn use_barrier() {

    let barrier = Arc::new(Barrier::new(5));
    let mut handlers = Vec::with_capacity(5);

    for i in 0..5 {
        let b = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("{i} before barrier");
            b.wait();

            // 所有的task打印before barrier后才会打印 after
            println!("{i} after barrier")
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("after barrier");
}

fn use_condvar() {

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_cloned = pair.clone();

    thread::spawn(move || {
        let &(ref locker, ref cvar) = &*pair_cloned;
        let mut started = locker.lock().unwrap();
        thread::sleep(time::Duration::from_secs(2));
        println!("started changing occur");
        *started = true;
        cvar.notify_one();
    });

    let &(ref locker, ref cvar) = &*pair;
    let mut started = locker.lock().unwrap();
    if !*started {
        // 当started没有置为true时，会一直阻塞在这里
        println!("wait at cvar.wait");
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

fn use_callonce() {

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    let mut handlers = Vec::with_capacity(5);
    for i in 0..5 {
        handlers.push(thread::spawn(move || {
            let mut rand = rand::thread_rng();
            thread::sleep(time::Duration::from_secs(rand.gen_range(1, 3)));
            
            println!("this is {i}, set VAL={i}");
            INIT.call_once(|| {
                unsafe {
                    VAL = i;   
                }
            });
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("VAL is {}", unsafe{VAL});
}

fn use_refcell() {

    thread_local! {
        static FOO: RefCell<u32> = RefCell::new(1);
    }

    // 这种方式读写值都需要unsafe
    static mut val:i32 = 1;

    FOO.with(|foo| {
        assert_eq!(*foo.borrow(), 1);
        *foo.borrow_mut() = 2;
        unsafe {
            println!("{val}");
            val = 10;
        }
    });

    thread::spawn(move || {
        FOO.with(|foo| {
            assert_eq!(*foo.borrow(), 1);
            *foo.borrow_mut() = 3;
            unsafe {
                println!("{val}");
            }
        });
    });

    FOO.with(|foo| {
        assert_eq!(*foo.borrow(), 2);
    });
}

// cargo run --bin about_std_sync
fn main() {

    // use_barrier();
    use_condvar();
    // use_callonce();
    // use_refcell();
}