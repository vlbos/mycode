use nix::unistd::{ForkResult, close, dup2};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::AsRawFd;
use std::process::{Command, Stdio};
use std::os::fd::FromRawFd;
use std::io::Read;
fn main() {
    // TCP
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to add");

    let (stream, _) = listener.accept().expect("Failed to accept connection");

    let socket_fd = stream.as_raw_fd();


    match unsafe { nix::unistd::fork() } {
        Ok(ForkResult::Parent { child }) => {
        
            println!("Parent process. Child PID: {}", child);
            drop(stream);
        }
        Ok(ForkResult::Child) => {
            drop(listener); 
        
            let mut child_stream = unsafe { TcpStream::from_raw_fd(socket_fd) };

            drop(stream); 
            let mut buffer = [0; 1024];
            match child_stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    println!(
                        "Child process. Received {} bytes: {:?}",
                        bytes_read,
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                }
                Err(e) => {
                    eprintln!("Child read error: {}", e);
                }
            }
        }
        Err(_) => {
            eprintln!("Fork failed");
        }
    }
}
