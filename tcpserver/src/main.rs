use std::net::TcpListener;
use std::io::{Read,Write};
use std::str;
use std::net::SocketAddr;
fn main() {
    let address: String = config();
    let listener: TcpListener =TcpListener::bind(&address).unwrap();
    println!("bind on {}",address);

    for stream in listener.incoming() {
	let mut stream = stream.unwrap();
	println!("connection established!");
	let mut buffer = [0; 100];

	let addr =  stream.peer_addr().unwrap();
	match addr {
	    SocketAddr::V4(a) => {
		let ip: &[u8] = &a.ip().octets();
		let port = a.port() as u8;
		println!("client address {}.{}.{}.{}:{:?}",ip[0],ip[1],ip[2],ip[3],port);
		let mut address = [0;5];
		for i in 0..4 {
		    address[i] = ip[i];
		}
		address[4] = port;
		stream.write(&address).unwrap();
	    }
	    SocketAddr::V6(b) => {
		let ip: &[u8] = &b.ip().octets();
		let port = b.port() as u8;
		println!("client address {:?}:{:?}",ip,port);
		let mut address = [0;5];
		for i in 0..4 {
		    address[i] = ip[i];
		}
		address[4] = port;
		stream.write(&address).unwrap();
	    }
	}
	stream.read(&mut buffer).unwrap();

    }
}

//parse command
fn config() -> String{
    use std::env;
    use std::env::Args;
    let args: Args = env::args();
    let mut vec: Vec<String> = vec!();
    for args in args {
	vec.push(args);
    }
    if vec.len() > 1 && vec.contains(&"--help".to_string()) {
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
