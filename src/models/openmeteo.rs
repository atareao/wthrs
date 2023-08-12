use serde_json::Value;
use anyhow::Error;

const URL: &'static str = "https://api.open-meteo.com";

pub struct OpenMeteoClient{
    latitude: String,
    longitude: String,
    timezone: String,
}

impl OpenMeteoClient{
    pub fn new(latitude: String, longitude: String, timezone: String) -> Self{
        Self{
            latitude,
            longitude,
            timezone,
        }
    }
    pub async fn get(self) -> Result<Value, Error>{
        let url = format!("{}/v1/forecast", URL);
        // https://api.open-meteo.com/v1/forecast?latitude=39.3626&longitude=-0.4117&hourly=temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,weathercode,pressure_msl,surface_pressure,cloudcover,visibility,evapotranspiration,windspeed_10m&daily=weathercode,temperature_2m_max,temperature_2m_min,sunrise,sunset,uv_index_max&current_weather=true&timezone=Europe%2FBerlin
        let params = [
            ("latitude", self.latitude.to_string()),
            ("longitude", self.longitude.to_string()),
            ("hourly", "temperature_2m,relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,weathercode,pressure_msl,surface_pressure,cloudcover,visibility,evapotranspiration,windspeed_10m".to_string()),
            ("daily", "weathercode,temperature_2m_max,temperature_2m_min,sunrise,sunset,uv_index_max".to_string()),
            ("current_weather", "true".to_string()),
            ("timezone", self.timezone),

        ];
        let client = reqwest::Client::new();
        let response = client.get(url)
            .query(&params)
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str(&response)?)
    }
}
