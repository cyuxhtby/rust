// Simple spawning of threads
fn threads() {
    use std::thread;
    use std::time::Duration;

    println!("threads()");
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} has started", i);
            thread::sleep(Duration::from_millis(1));
            println!("Thread {} is finished", i); 
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("");
}

// Producer-Consumer Pattern:
// Tasks are prepared in parallel but processed in order of completion 
fn threads_ordered_processing() {
    use std::thread;
    use std::time::Duration;
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    println!("threads_ordered_processing()");
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut handles = vec![];
    
    // Producer threads
    for i in 0..5 {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            println!("Producer thread {} has started", i);
            thread::sleep(Duration::from_millis(200 * i as u64)); // Stagger the sleep times
            // Acquire lock, then push to queue
            queue.lock().unwrap().push_back(i);
        }))
    }

    // Consumer thread
    handles.push(thread::spawn({
        // The threads in queue are already ordered due to simulated sleep times
        let queue = Arc::clone(&queue);
        move || {
            for _ in 0..5 {
                loop {
                    if let Some(item) = queue.lock().unwrap().pop_front() {
                        thread::sleep(Duration::from_millis(1));
                        println!("Consumer finished processing item {}", item);
                        break;
                    }
                // Short sleep to prevent busy-waiting
                thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}

// A note on closures:
// - Anonymous functions that capture their environment
// - Syntax: |params| { body }
// - Used here to define thread behavior inline
//   and capture variables (i, queue) from loop scope.
// - `move` transfers ownership of captured variables to the closure

fn main() {
    threads();
    threads_ordered_processing();
}