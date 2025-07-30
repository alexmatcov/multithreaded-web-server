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
    let http_request: Vec<_> = buf_reader
        // returns an iterator of Result<String, std::io::Error> by splitting the stream at a newline byte 
        .lines()
        // get each String by unwrap() each Result
        .map(|result| result.unwrap())
        // browser signals the end of an HTTP request by sending 2 newline chars in a row
        .take_while(|line| !line.is_empty())
        .collect();
        
    println!("Request: {http_request:#?}");

}
