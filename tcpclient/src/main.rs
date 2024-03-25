use std::net::TcpStream;
use std::io::{Read,Write};
use std::str;
fn main() {
    let address: String = config();
    let mut stream:TcpStream = TcpStream::connect(&address).unwrap();
    stream.write("你好你的ip是".as_bytes()).unwrap();
    let mut buffer = [0;50];
    stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).unwrap();
    println!("address{}.{}.{}.{}:{}",&buffer[0],&buffer[1],&buffer[2],&buffer[3],&buffer[4]);
}

fn config() -> String{
    use std::env;
    use std::env::Args;
    let args: Args = env::args();
    let mut vec: Vec<String> = vec!();
    for args in args {
	vec.push(args);
    }
    if vec.len() > 1 && vec[1].contains("--help") {
	help(&vec);
    }

    if vec.len() < 2 {
	help(&vec);
    }
    let server_address: String = env::args().nth(1).unwrap();
    println!("server_address is {:?}",server_address);
    server_address
}

fn help(vec: &Vec<String>) ->(){
    println!("Usage: {} server_address", vec[0]);
    println!("Example: {} 0.0.0.0:3000", vec[0]);
    std::process::exit(0);
}
