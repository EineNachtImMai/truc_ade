pub mod caching;
pub mod calendar_parsing;
pub mod cli_params;
pub mod free_rooms;
pub mod networking;
pub mod utils;

use free_rooms::serve_free_rooms;

// NOTE: The ADE cal goes from 6h to 21h

#[tracing::instrument]
#[tokio::main]
async fn main() {
    let sub = tracing_subscriber::FmtSubscriber::new();
    match tracing::subscriber::set_global_default(sub) {
        Ok(_) => {tracing::info!("Successfully set up tracing!")},
        Err(_) => {eprintln!("Oh no, no tracing :(")},
    }
    serve_free_rooms().await;
}
