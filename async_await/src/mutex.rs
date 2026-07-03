/**
 *
 * Arc support multiple owners
 * Mutex supports one writer at a time
 */
use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_concept() {
    let counter = Arc::new(Mutex::new(0));

    let counter2 = Arc::clone(&counter);

    let handler = thread::spawn(move || {
        let mut c = counter2.lock().expect("Issue While Locking");
        *c = *c + 1;

        println!("Worker Thread: {}", *c);
    });

    // wait till the Worker Thread Completes
    handler.join().unwrap();

    let c = counter.lock().unwrap();
    println!("Main Thread: {}", *c);
}
