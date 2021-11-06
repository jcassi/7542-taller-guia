use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

//use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::SocketAddr;
use std::io::Read;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    
    let handle = thread::spawn(|| {
        client_thread();
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server_handle_connection(stream);
    }

    handle.join().unwrap();
}

fn server_handle_connection(mut stream: TcpStream) {
    let message = read_message(&mut stream);
    print_message(message);
    send_message("Buen dia hijo", &mut stream)
}

fn client_thread() {
    //let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7877);
    let addrs = [
        SocketAddr::from(([0, 0, 0, 0], 7878)),
    ];
    if let Ok(stream) = TcpStream::connect(&addrs[..]) {
        client_handle_connection(stream);
    } else {
        println!("No se pudo conectar...");
    }
}

fn client_handle_connection(mut stream: TcpStream) {
    send_message("Buen dia padre", &mut stream);
    let message = read_message(&mut stream);
    print_message(message);
}

fn send_message(message: &str, stream: &mut TcpStream) {
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn read_message(stream: &mut TcpStream) -> [u8; 1024]{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    buffer
}

fn print_message(buffer: [u8; 1024]) {
    println!("{}", String::from_utf8_lossy(&buffer[..]));
}
