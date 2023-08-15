use std::error;

use super::openmeteo::OpenMeteoClient;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// Open Meteo Client
    pub omc: OpenMeteoClient<'a>
}


impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(latitude: &'a str, longitude: &'a str, timezone: &'a str) -> App<'a> {

        Self {
            running: true,
            counter: 0,
            omc: OpenMeteoClient::new(latitude, longitude, timezone),

        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
