use std::thread;
// Multiple producer, Single consumer (thread msg passing)
// Allows the creation of channels consisting of a Sender/Receiver pair
// Sender<T> allows msg passing to Receiver<T>
use std::sync::mpsc;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    // We reduce n by its square root since
    // any divisor greater than the sqrt would have a corresponding smaller divisor already checked
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_primes(max: u32) {

    // Define sender and receiver
    let (tx, rx) = mpsc::channel();

    for n in 1..=max {
        // Each thread needs its own sender
        let tx = tx.clone();

        thread::spawn(move || {
            if is_prime(n) {
                tx.send(n).expect("Uhhhh")
            }
        });
    }

    drop(tx); // Close sending end

    for prime in rx {
        println!("{} is prime", prime);
    }
}

fn main() {
    find_primes(60);
}