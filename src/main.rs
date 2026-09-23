use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub mod hello;

use crate::hello::print_hello;

#[tokio::main]
async fn main() {
    print_hello();
}
