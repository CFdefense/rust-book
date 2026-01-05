use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_threaded_mutex() {
        // Create a new mutex with the value 5
        let m = Mutex::new(5);

        {
            // To access the data inside the mutex, we use the lock method to acquire the lock.
            let mut num = m.lock().unwrap();

            // Dereference the mutable reference to set the value to 6
            *num = 6;
        }

        println!("m = {m:?}");
    }

    #[test]
    fn shared_state_mutex() {
        // Create a new mutex with the value 0
        // Arc is an atomic reference-counted pointer
        let counter = Arc::new(Mutex::new(0));

        // Create a vector to store the handles
        let mut handles = vec![];

        // Create 10 threads
        for _ in 0..10 {
            // Clone the counter to create a new counter
            let counter = Arc::clone(&counter);
            // Spawn a new thread
            let handle = thread::spawn(move || {
                // Acquire a lock on the counter
                let mut num = counter.lock().unwrap();

                // Dereference the mutable reference to add 1 to the value
                *num += 1;
            });
            // Add the handle to the vector
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}


fn main() {
    println!("Hello, world!");
}
