extern crate env_logger;
#[cfg(feature = "futuring")]
extern crate futures;
extern crate ticketed_lock;

#[cfg(feature = "futuring")]
use futures::Future;

use std::thread;
use ticketed_lock as tl;

fn main() {
    env_logger::init();

    let mut storage = tl::TicketedLock::new(4u8);
    let t1 = storage.read();
    let t2 = storage.read();
    let t3 = storage.write();

    let g3 = thread::spawn(move|| {
        let mut guard = t3.wait().expect("failed to wait on t3");
        *guard += 1;
        println!("t3: {}", *guard);
    });
    let g2 = thread::spawn(move|| {
        let guard = t2.wait().expect("failed to wait on t2");
        println!("t2: {}", *guard);
    });
    let g1 = thread::spawn(move|| {
        let guard = t1.wait().expect("failed to wait on t1");
        println!("t1: {}", *guard);
    });

    g1.join().unwrap();
    g2.join().unwrap();
    g3.join().unwrap();
}
