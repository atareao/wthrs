mod models;

use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use clap::Parser;
use std::error::Error;

use models::{
    args::Cli,
    app::App,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};


type Result<T> = std::result::Result<T, Box<dyn Error>>;


fn main() -> Result<()>{
    let cli = Cli::parse();
    println!("{:?}", cli);
    let lat = cli.latitude;
    let lon = cli.longitude;
    let tz = cli.timezone;

    // Create applicaiton
    let mut app = App::new(&lat, &lon, &tz);

    // Init the terminal user interface
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        let _ = tui.draw(&mut app);
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
