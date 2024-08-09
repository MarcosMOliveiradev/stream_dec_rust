pub mod server;
pub mod router{
    pub mod controller;
}
#[tokio::main]
async fn main() {
    server::server().await;
}
