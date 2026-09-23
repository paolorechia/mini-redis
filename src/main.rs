use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub mod hello;

use crate::hello::print_hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Listening on 127.0.0.1:8080");
        tokio::spawn(async move {
            println!("Spawning...");
            let mut buf = [0; 64];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("Got: {}", n);
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
