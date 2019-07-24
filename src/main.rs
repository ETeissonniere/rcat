#[macro_use]
extern crate clap;

use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::io;

fn main() {
    let matches = clap_app!(rcat => 
        (version: "1.0")
        (author: "Eliott Teissonniere <eliott.teissonniere.org>")
        (about: "Tiny netcat like program in rust")
        (@arg PORT: -p --port +takes_value "Port to connect to or listen on")
        (@arg ADDR: -a --address +takes_value "Address to connect to or listen on")
        (@subcommand listen => 
            (about: "Listen for incoming connections on specified address and port")
        )
        (@subcommand connect =>
            (about: "Connect to specified address and port")
        )
    ).get_matches();

    let addr = matches.value_of("ADDR").unwrap_or("127.0.0.1");
    let port: u16 = matches.value_of("PORT").unwrap_or("4242").parse().unwrap();
    
    if let Some(_) = matches.subcommand_matches("listen") {
        println!("Listening on {}:{}", addr, port);
        listen(addr, port);
    } else if let Some(_) = matches.subcommand_matches("connect") {
        println!("Connecting to {}:{}", addr, port);
        connect(addr, port);
    } else {
        println!("Unknown command");
    }
}

fn connect(addr: &str, port: u16) {
    let socket = SocketAddr::new(IpAddr::V4(addr.parse().unwrap()), port);
    if let Ok(mut stream) = TcpStream::connect(&socket) {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdin().read_line(&mut input).unwrap(); // Unwrap is used to ensure no errors occured
            println!(">>> {}", input.trim());

            stream.write(input.as_bytes()).unwrap(); // Same for unwrap
        }
    } else {
        println!("Failure to connect to the target");
    }
}

fn listen(addr: &str, port: u16) {
    match TcpListener::bind((addr, port)) {
		Ok(listener) => {
			for stream in listener.incoming() {
				let stream = stream.unwrap();
				handle_connection(stream);
			}
		},
		Err(e) => {println!("Error: {}", e)}
	}
}

fn handle_connection(mut stream: TcpStream){
	loop {
		let mut buffer = [0; 512];
		match stream.read(&mut buffer) {
			Ok(s) => {
				if s == 0 { return }

				let received = String::from_utf8_lossy(&buffer[..]);
				print!("<<< {}", received);
			},
			Err(e) => {println!("Error: {}", e)}
		}
	}
}
