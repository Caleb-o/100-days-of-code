/*
    100 DAYS OF CODE: Day 002
    Fearless Concurrency - Rust Book
*/
use std::{
    thread,
    time::Duration,
    sync::{Arc, mpsc, Mutex},
};

fn main()
{
    // Threading
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        println!("A vector: {:?}", v);
    });

    let handle = thread::spawn(|| {
        for i in 1..=10
        {
            println!("Hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..=5
    {
        println!("Hi number {} from spawned thread", i);
        thread::sleep(Duration::from_millis(1));
    }


    // Communicate through channels
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals
        {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx
    {
        println!("Got: {}", received);
    }


    // Mutex to use one thread at a time
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    
    // Sharing mutexes
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles
    {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}