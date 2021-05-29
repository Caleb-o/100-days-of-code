/*
    100 DAYS OF CODE : Day 9
    Iterators and then Some
*/
use std::{
    thread,
    time::Duration,
};

fn get_an_option() -> Option<i32>
{
    Some(5)
}

fn get_a_result(x: i32) -> Result<(), String>
{
    if x >= 10
    {
        Ok(())
    }
    else
    {
        Err(String::from("X is less than 10"))
    }
}

fn main()
{
    // Creating vecs
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v2: Vec<i32> = v1.iter().map(|val| val + 1).collect();
    let v3: Vec<&i32> = v2.iter().filter(|val| *val % 2 == 0).collect();

    // Printing our iterators
    println!("Vec1 = {:?}", v1);
    println!("Vec2 = {:?}", v2);
    println!("Vec3 = {:?}", v3);
    println!("Vec3 sum = {}", v3.into_iter().sum::<i32>());


    // Somes
    let x = Some(5);
    let y = 10;

    match x
    {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {:?}", y),
        _ => println!("Default = {:?}", x),
    }

    println!("Both : x = {:?}, y = {:?}", x, y);


    // Options and Results
    if let Some(val) = get_an_option()
    {
        println!("Our value : {}", val);
    }

    for x in (1..=3).into_iter()
    {
        match get_a_result(x * 5)
        {
            Ok(_) => println!("Successful"),
            Err(e) => println!("Error: {}", e),
        }
    }


    // Closures
    let square = |x| x * x;
    println!("5 squared = {}", square(5));

    let wait_for_time = |seconds: u64|
    {
        println!("Waiting for {} second(s)...", seconds);
        thread::sleep(Duration::from_secs(seconds));
    };

    wait_for_time(1);

    let build_things = |seconds: u64, name: String| -> i32
    {
        wait_for_time(seconds);
        println!("{} has been built successfully!", name);

        10
    };

    let result = build_things(3, String::from("Cool Project"));
    println!("Result from project: {}", result);
}