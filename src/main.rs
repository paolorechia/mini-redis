
pub mod connection_manager;

use crate::connection_manager::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    return init().await;
}
