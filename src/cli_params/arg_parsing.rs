use clap::Parser;

/// An API to find empty classrooms
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The port on which to expose the API (default 7878)
    #[arg(short, long, default_value_t = 7878)]
    pub port: u16,

    /// The time period covered by the free rooms API
    /// Measured in days
    #[arg(short, long, default_value_t = 3)]
    pub free_rooms_timespan: u16,

    /// The number of weeks covered by the Zik API
    /// Measured in weeks
    #[arg(short, long, default_value_t = 2)]
    pub zik_timespan: u16,
}
