pub mod connection_manager;
pub mod frame_controller;

use crate::connection_manager::ConnectionManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result_connection_manager = ConnectionManager::init("127.0.0.1:8080".to_string()).await;
    match result_connection_manager {
        Ok(mut connection_manager) => connection_manager.serve().await,
        Err(err) => Err(err),
    }
}
