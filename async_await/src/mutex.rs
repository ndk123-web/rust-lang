/**
 *
 * Arc support multiple owners
 * Mutex supports one writer at a time
 *
 * We have 2 version of Mutex
 * 1. std::sync::Mutex
    - Assume 1 Tokio worker thread.
    - Task A acquires the mutex.
    - Task B tries to acquire the same mutex using lock().
    - The worker thread gets BLOCKED.
    - Because the thread is blocked, Task C (even if it doesn't need the mutex)
    also cannot run.

    Result:
        A -> holds lock
        B -> blocks the thread
        C -> cannot execute


 * 2. tokio::sync::Mutex

    - Assume 1 Tokio worker thread.
    - Task A acquires the mutex.
    - Task B tries to acquire the same mutex using lock().await.
    - Task B is suspended (not the thread).
    - Tokio scheduler can now run Task C if it is ready.

    Result:
        A -> holds lock
        B -> waiting for mutex
        C -> continues executing
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
