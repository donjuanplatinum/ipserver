use std::net::TcpStream;
use std::io::{Read,Write};
use std::str;
fn main() {
    let mut stream:TcpStream = TcpStream::connect("101.33.224.61:4000").unwrap();
    stream.write("你好你的ip是".as_bytes()).unwrap();
    let mut buffer = [0;50];
    stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).unwrap();
    println!("address{}.{}.{}.{}:{}",&buffer[0],&buffer[1],&buffer[2],&buffer[3],&buffer[4]);
}
