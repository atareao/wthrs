mod models;

use models::{
    args::Cli,
    terminal::TUI,
};
use clap::Parser;
use ratatui::Terminal;
use tokio;
use std::error::Error;


type Result<T> = std::result::Result<T, Box<dyn Error>>;


#[tokio::main]
async fn main() -> Result<()>{
    let cli = Cli::parse();
    println!("{:?}", cli);
    let lon = cli.longitude;
    let lat = cli.latitude;
    let tz = cli.timezone;
    let mut terminal = Terminal::setup()?;
    let result = terminal.run(lon, lat, tz).await;
    terminal.restore()?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}
