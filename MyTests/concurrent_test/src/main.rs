use std::thread;
use std::sync::mpsc; // multiple producer, single consumer
use std::sync::{Mutex, Arc};

fn main() {
    let simp_handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    simp_handle.join().unwrap();
	
	let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from a thread!").unwrap();
    });

    let message = rx.recv().unwrap();
    println!("Received: {}", message);
	
	let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

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

    println!("Result: {}", *counter.lock().unwrap());
}
