use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread
    // Move the transmitter into the closure so that the spawned thread owns it
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // Send the values through the channel
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receive the values from the channel
    for received in rx {
        println!("Got: {received}");
    }

    // --snip--

    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter to create a new transmitter
    let tx1 = tx.clone();

    // Spawn a new thread
    // Move the transmitter into the closure so that the spawned thread owns it
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // Send the values through the channel
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawn a new thread
    // Move the transmitter into the closure so that the spawned thread owns it
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        // Send the values through the channel
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receive the values from the channel
    for received in rx {
        println!("Got: {received}");
    }
}