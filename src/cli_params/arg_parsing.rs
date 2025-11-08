use clap::Parser;

/// An API to find empty classrooms
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The port on which to expose the API (default 7878)
    #[arg(short, long, default_value_t = 7878)]
    pub port: u16,
}
