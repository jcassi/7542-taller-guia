use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::TcpStream;
use std::io::Read;
use std::io::prelude::*;

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 59, 128)), 7877);
    let addrs = [
        SocketAddr::from(([192, 168, 59, 128], 7878)),
    ];
    if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
        let request = "Buen dia padre";
        stream.write(request.as_bytes()).unwrap();
        stream.flush().unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer[..]));
    } else {
        println!("No se pudo conectar...");
    }
}

