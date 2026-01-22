use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_thread() {
        // Spawn a new thread
        thread::spawn(|| {
            // Print messages from the spawned thread
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        // Print messages from the main thread
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn joins_thread() {
        // Spawn a new thread
        // Handle is a JoinHandle<T>
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        // Print messages from the main thread
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    
        // Join the thread
        // This will block the main thread until the spawned thread finishes
        handle.join().unwrap();
    }

    #[test]
    fn moves_ownership() {
        // Create a new vector
        let v = vec![1, 2, 3];

        // Spawn a new thread
        // Move the vector into the closure so that the spawned thread owns it
        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        // Join the thread
        // This will block the main thread until the spawned thread finishes
        handle.join().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}