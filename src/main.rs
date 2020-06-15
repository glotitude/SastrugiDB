use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, BufRead};


fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let lines = reader.by_ref().lines();

    for line in lines {
        let l = line.unwrap();

        if l == String::from("") {
            break;
        }

        println!("{}", l);
    };
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
