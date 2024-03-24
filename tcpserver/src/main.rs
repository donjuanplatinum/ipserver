use std::net::TcpListener;
use std::io::{Read,Write};
use std::str;
use std::net::SocketAddr;
fn main() {
    let listener: TcpListener =TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("bind on 0.0.0.0:3000");

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

