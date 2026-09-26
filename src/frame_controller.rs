use std::cmp::min;
use std::fmt::{self, Debug, Formatter};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};

pub struct FrameController<T: AsyncRead + AsyncWrite + Debug + Unpin> {
    tcp_stream: T,
    frame_queue: Vec<String>,
}

impl<T: AsyncRead + AsyncWrite + Debug + Unpin> FrameController<T> {
    pub async fn init(tcp_stream: T) -> FrameController<T> {
        return FrameController {
            tcp_stream,
            frame_queue: Vec::<String>::new(),
        };
    }

    pub async fn control_stream(&mut self) {
        println!("Spawning... {:?}", self.tcp_stream);
        let mut buf = [0; 64];
        let mut num_read_bytes: u64 = 0;
        let mut found_start_marker: bool = false;
        let mut found_end_marker: bool = false;
        let mut found_key_marker: bool = false;
        let mut found_value_market: bool = false;
        let mut command: Vec<String>;
        let mut key: Vec<String>;
        let mut value: Vec<String>;

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
            println!("Got: {}, {:?}", n, buf);

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

    struct FakeTcpStream {
        fake_input_data: Vec<u8>,
        input_current_idx: usize,
    }

    impl FakeTcpStream {
        async fn init(fake_input_data: Vec<u8>) -> FakeTcpStream {
            return FakeTcpStream {
                fake_input_data,
                input_current_idx: 0,
            };
        }
    }

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

    impl AsyncRead for FakeTcpStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<Result<(), Error>> {
            // ReadBuffer
            // [FILLED / REMAINING]
            let available_space = buf.remaining();

            // Available remaining fake data to read
            let available_data = self.fake_input_data.len() - self.input_current_idx;

            // Calculate how much we can insert given available data / available space in ReadBuf
            let slice_upper_index = self.input_current_idx + min(available_data, available_space);

            // Create the slice
            let fake_data_slice_to_append =
                &self.fake_input_data[self.input_current_idx..slice_upper_index];

            // Insert
            buf.put_slice(fake_data_slice_to_append);

            // Update cursor for next poll_read call
            self.input_current_idx = slice_upper_index;

            // Signal we're done with this Poll
            return Poll::Ready(Ok(()));
        }
    }

    impl Debug for FakeTcpStream {
        fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            return Ok(());
        }
    }

    async fn init_controller_with_data(data_as_string: &str) -> FrameController<FakeTcpStream> {
        let tcp_stream = FakeTcpStream::init(data_as_string.bytes().collect()).await;
        return FrameController::init(tcp_stream).await;
    }

    #[tokio::test]
    async fn test_invalid_message() {
        let mut frame_controller: FrameController<FakeTcpStream> =
            init_controller_with_data("Hello world!").await;

        frame_controller.control_stream().await;
        assert_eq!(frame_controller.frame_queue.len(), 0);
    }

    #[tokio::test]
    async fn test_hello_world_message() {
        let mut frame_controller: FrameController<FakeTcpStream> = init_controller_with_data(
            "#####START#####@@@@@KEY@@@@@hello world key@@@@@VALUE@@@@@hello world value#####END#####",
        ).await;
        frame_controller.control_stream().await;
        assert_eq!(frame_controller.frame_queue.len(), 1);
    }
}
