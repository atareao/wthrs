use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Latitude
    #[arg(short = 't', long, value_name = "Latitude", allow_negative_numbers = true)]
    pub latitude: String,
    
    /// Longitude
    #[arg(short = 'n', long, value_name = "Longitude", allow_negative_numbers = true)]
    pub longitude: String,

    /// Timezone
    #[arg(short = 'z', long, value_name = "Timezone", default_value = "Europe/Madrid")]
    pub timezone: String,
}
