/* 
    100 DAYS OF CODE : Day 8
    Rust Ownership
*/

fn takes_ownership(some_string: String)
{
    println!("Taken ownership: {}", some_string);
}

fn take_and_return(some_string: String) -> String
{
    some_string
}

fn get_length(s: String) -> (String, usize)
{
    let length = s.len();

    (s, length)
}

fn get_length_ref(s: &String) -> usize
{
    s.len()
}

fn change(s: &mut String)
{
    s.push_str(", World!");
}

fn makes_copy(some_int: i32)
{
    println!("Copied: {}", some_int);
}

fn main()
{
    // Some types, like numbers, will be copied by default
    let x = 5;
    let y = x;
    println!("Value of x: {}, y: {}", x, y);

    let b = true;
    let n = b;
    println!("Value of b: {}, n: {}", b, n);


    // Strings on the other hand, will not be copied, they will be moved
    let s1 = String::from("Hello");
    let s2 = s1; // Move occurs

    // If we called this, we would get a move error
    //println!("String value: {}", s1);
    println!("String value: {}", s2);
    
    // If we want to copy a String we can clone it
    let s1 = String::from("Indeed");
    let s2 = s1.clone();
    println!("String value: '{}' and '{}'", s1, s2);


    // Other values that can be copied:
    // floating point, char, any tuple that implements the Copy trait
    // (i32, i32)
    let tup1: (i32, i32) = (10, 20);
    let tup2 = tup1;
    println!("T1 = {:?}, T2 = {:?}", tup1, tup2);
    
    
    // This will need a clone since it contains a String
    let tup1: (i32, String) = (30, String::from("Woah"));
    let tup2 = tup1.clone();
    println!("T1 = {:?}, T2 = {:?}", tup1, tup2);


    // The same rules apply to functions
    makes_copy(tup1.0);         // i32 will be copied
    takes_ownership(tup1.1);    // String will be moved

    // Since the String was moved, it cannot be used again
    //println!("Use string: {}", tup.1);

    // We can pass ownership and get it back
    let s1 = take_and_return(tup2.1);
    println!("S1 = {}", s1);

    let tup1 = get_length(s1);
    println!("T1 = {:?}", tup1);

    // We can mitiage moving by just referencing
    let sz = get_length_ref(&tup1.0);
    println!("Size of '{}' is: {}", tup1.0, sz);

    let mut s1 = take_and_return(tup1.0);

    // If we wanted to modify the variable, we would have to mutably borrow
    change(&mut s1);
    println!("New S1 = {}", s1);
    
    
    // Rust will let you immutably borrow a variable several times,
    // but you can only mutably borrow a variable once at a time
    let mut s = String::from("Hello");
    
    {
        let r1 = &mut s;
        r1.push_str(", World!");
    } // r1 goes out of scope, we can borrow again
    println!("New S = {}", s);
    
    
    //let r2 = &mut s;

    // The same issue occurs when mixing immutable and mutable borrows
    //let r1 = &s;
    //let r2 = &s;
    //let r3 = &mut s; // Error

    let r1 = &s;
    let r2 = &s;
    println!("Immutable borrow: '{}' and '{}'", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;
    println!("Mutable borrow: '{}'", r3);
}