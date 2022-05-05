use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let now = Instant::now();

    //     let mut counter = 0;
    // for _ in 0..10 {
    //     counter += 1;
    // }

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // println!("Result: {}", counter);
    println!("Result: {}", *counter.lock().unwrap());
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
