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

        println!("Connection established!");
    }
}
