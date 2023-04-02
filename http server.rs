use std::net::TcpListener;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
    stream.write(response.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
