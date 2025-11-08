pub mod free_rooms;
pub mod calendar_parsing;
pub mod cli_params;
pub mod networking;

use free_rooms::{serve_free_rooms};

// NOTE: The ADE cal goes from 6h to 21h

fn main() {
    serve_free_rooms();
}
