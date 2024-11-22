use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("Client connected: {}", stream.peer_addr()?);

    // Thread to read from server
    let mut read_stream = stream.try_clone()?;
    thread::spawn(move || {
        let reader = io::BufReader::new(read_stream);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("> {}", line),
                Err(e) => {
                    eprintln!("Failed to read from client: {}", e);
                    break;
                }
            }
        }
    });

    // Main thread to read from user input and send to client
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        stream.write_all(line.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9088")?;
    println!("await");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}