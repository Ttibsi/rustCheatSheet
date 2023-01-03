use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn simple_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number in thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    thread::spawn(move || {
        // use the move keyword to let a closure take ownership of a variable
        // so it can exist within a thread
        println!("Here's a vec: {:?}", v);
    });

    for i in 1..5 {
        println!("number in main: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // Block the main thread from ending until the handle thread has finished
    handle.join().unwrap();
}

fn channels() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let foo = String::from("hi");
        tx1.send(foo).unwrap(); // send message via channel
    });

    thread::spawn(move || {
        let foo = String::from("hello");
        tx.send(foo).unwrap(); // send message via channel
    });

    // let rec = rx.recv().unwrap();
    for received in rx {
        println!("Message received: {}", received);
    }
}

fn main() {
    channels()
}
