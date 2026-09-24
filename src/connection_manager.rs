use crate::frame_controller::FrameController;
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
            let (socket, _) = self.listener.accept().await?;
            println!("Listening on {}", self.endpoint);
            tokio::spawn(async move {
                let mut frame_controller = FrameController::init(socket).await;
                frame_controller.control_stream().await;
            });
        }
    }
}
