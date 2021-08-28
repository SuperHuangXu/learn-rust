use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;

fn main() {
    // thread_demo();
    // channel_demo();
    shared_state_demo();
}

fn thread_demo() {
    let v = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

// 通过通信实现并发
fn channel_demo() {
    let (sender, receive) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for x in v {
            sender.send(x).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for x in receive {
        println!("Got: {}", x);
    }
}

// 共享状态并发
fn shared_state_demo() {
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

    println!("Result {}", *counter.lock().unwrap());
}
