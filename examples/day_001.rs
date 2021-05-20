use std::{
    fs,
    thread,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

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

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}