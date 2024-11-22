use ::std::io::{Read, Write};
use ::std::str;
use std::net::TcpStream;
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello there.".as_bytes()).unwrap();
    let mut buffer = [0; 12];
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
