/*
    100 DAYS OF CODE: Day 001
    Webserver - Rust book

    The first day is coming in with something more than a few functions and language testing.
    I decided to follow the Rust book earlier and thought it would be nice to work through
    the last section as my first day of code. I still have a long way to go with Rust and
    would love to do more with this little app.

    I was able to understand *most* of the project code and felt confident writing the code.
*/

use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use one_hundred_days_of_code::webclient::ThreadPool;


fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_req: &[u8] = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_req)
    {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Load relevant HTML file
    let contents: String = fs::read_to_string(filename).unwrap();

    // Build a response
    let response: String = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main()
{
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}