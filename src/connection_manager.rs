use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct ConnectionManager {
    listener: TcpListener,
    endpoint: String,
}

impl ConnectionManager {
    pub async fn init(endpoint: String) -> Result<ConnectionManager, Box<dyn std::error::Error>> {
        return Ok(ConnectionManager {
            listener: TcpListener::bind(&endpoint).await?,
            endpoint: endpoint,
        });
    }
    pub async fn serve(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (mut socket, _) = self.listener.accept().await?;
            println!("Listening on {}", self.endpoint);
            tokio::spawn(async move {
                println!("Spawning... {:?}", socket);
                let mut buf = [0; 64];

                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(0) => {
                            println!("Connection closed: {:?}", socket);
                            return;
                        }
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
}
