use crate::app::App;

pub mod app;
pub mod routes;
pub mod models;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
