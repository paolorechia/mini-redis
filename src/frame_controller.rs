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
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::io::Error;

    struct FakeTcpStream {}
    impl AsyncWrite for FakeTcpStream {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            return Poll::Ready(Ok(buf.len()));
        }
        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
            return Poll::Ready(Ok(()));
        }
        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
            return Poll::Ready(Ok(()));
        }
    }
    // impl AsyncRead for FakeTcpStream {
    //     fn poll_read(
    //         self: Pin<&mut Self>,
    //         cx: &mut Context<'_>,
    //         buf: &mut ReadBuf<'_>,
    //     ) -> Poll<Result<()>> {
    //         return Poll::Ready(Ok);
    //     }
    // }

    #[test]
    fn test_example() {}
}
