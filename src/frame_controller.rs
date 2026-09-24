use std::fmt::Debug;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct FrameController<T: AsyncRead + AsyncWrite + Debug + Unpin> {
    tcp_stream: T,
}

impl<T: AsyncRead + AsyncWrite + Debug + Unpin> FrameController<T> {
    pub async fn init(tcp_stream: T) -> FrameController<T> {
        return FrameController { tcp_stream };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // FrameController
    }
}
