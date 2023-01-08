use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // This is our loop to check for incoming data fromm the client
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        handle_conn(stream)
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "www/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "www/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(resp.as_bytes()).unwrap();
}
