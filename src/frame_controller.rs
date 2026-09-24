use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct FrameController {
    tcp_stream: TcpStream,
}

impl FrameController {
    pub async fn init(tcp_stream: TcpStream) -> FrameController {
        return FrameController {
            tcp_stream: tcp_stream,
        };
    }

    pub async fn control_stream(&mut self) {
        println!("Spawning... {:?}", self.tcp_stream);
        let mut buf = [0; 64];

        loop {
            let n = match self.tcp_stream.read(&mut buf).await {
                Ok(0) => {
                    println!("Connection closed: {:?}", self.tcp_stream);
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket; err = {:?}", e);
                    return;
                }
            };
            println!("Got: {}", n);
            if let Err(e) = self.tcp_stream.write_all(&buf[0..n]).await {
                eprintln!("Failed to write socket; err = {:?}", e);
                return;
            }
        }
    }
}
