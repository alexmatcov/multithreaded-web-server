use std::{
    // bring filesystem into scope
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    // bind() is creating a new instance 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming() is an iterator over connection attempts
    for stream in listener.incoming() {
        // unwrap() terminates the program if the stream has errors
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // first unwrap() takes care of the option and takes care if the iterator has no items,
    // the second take the String from the Result
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // as_bytes() converts the string data to bytes.
        // write_all() takes a &[u8] and sens the bytes directly down the connection
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }

}
