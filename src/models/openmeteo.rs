use serde_json::Value;
use anyhow::Error;

const URL: &'static str = "https://api.open-meteo.com";

#[derive(Debug)]
pub struct OpenMeteoClient<'a>{
    latitude: &'a str,
    longitude: &'a str,
    timezone: &'a str,
}

impl<'a> OpenMeteoClient<'a>{
    pub fn new(latitude: &'a str, longitude: &'a str, timezone: &'a str) -> OpenMeteoClient<'a>{
        Self{
            latitude,
            longitude,
            timezone,
        }
    }
    pub fn get(&self) -> Result<Value, Error>{
        let url = format!("{}/v1/forecast", URL);
        // https://api.open-meteo.com/v1/forecast?latitude=39.3626&longitude=-0.4117&hourly=temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,weathercode,pressure_msl,surface_pressure,cloudcover,visibility,evapotranspiration,windspeed_10m&daily=weathercode,temperature_2m_max,temperature_2m_min,sunrise,sunset,uv_index_max&current_weather=true&timezone=Europe%2FBerlin
        let params = [
            ("latitude", self.latitude),
            ("longitude", self.longitude),
            ("hourly", "temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,weathercode,pressure_msl,surface_pressure,cloudcover,visibility,evapotranspiration,windspeed_10m"),
            ("daily", "weathercode,temperature_2m_max,temperature_2m_min,sunrise,sunset,uv_index_max"),
            ("current_weather", "true"),
            ("timezone", self.timezone),

        ];
        let client = reqwest::blocking::Client::new();
        let response = client.get(url)
            .query(&params)
            .send()?
            .text()?;
        Ok(serde_json::from_str(&response)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::openmeteo::OpenMeteoClient;

    #[test]
    fn test_openmeteo() {
        let omc = OpenMeteoClient::new("39.36667", "-0.41667", "Europe/Madrid");
        let data = omc.get();
        assert_eq!(data.is_ok(), true);
    }
}
