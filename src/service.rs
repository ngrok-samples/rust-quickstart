use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Read from the stream
    if let Ok(_) = stream.read(&mut buffer) {
        let body = "Hello from Rust HTTP Server!";
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Length: {}\r\n\
             Content-Type: text/plain\r\n\
             Connection: close\r\n\r\n\
             {}",
            body.len(),
            body
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Write error: {}", e);
        }

        // Flush to make sure the response is sent
        if let Err(e) = stream.flush() {
            eprintln!("Flush error: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
