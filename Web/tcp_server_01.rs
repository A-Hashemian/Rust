use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
       let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read from socket: {}", e);
                break;
            }
        };
    
      if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
            println!("Failed to write to socket: {}", e);
            break;
        }
    }
}