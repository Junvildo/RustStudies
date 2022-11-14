use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut thread = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        thread.push(t);
    }

    for th in thread {
        th.join().unwrap();
    }

    println!("Result {}", *c.lock().unwrap());
}
