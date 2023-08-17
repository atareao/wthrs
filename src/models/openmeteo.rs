use serde_json::Value;
use anyhow::Error;

const URL: &'static str = "https://api.open-meteo.com";

#[derive(Debug)]
pub struct OpenMeteoClient<'a>{
    latitude: &'a str,
    longitude: &'a str,
    timezone: &'a str,
    data: Option<Value>,
}

impl<'a> OpenMeteoClient<'a>{
    pub fn new(latitude: &'a str, longitude: &'a str, timezone: &'a str) -> OpenMeteoClient<'a>{
        Self{
            latitude,
            longitude,
            timezone,
            data: None,
        }
    }

    pub fn refresh(&mut self){
        match self.get(){
            Ok(value) => self.data = Some(value),
            Err(_) => self.data = None,
        }
    }

    pub fn get_data(&self) -> Option<Value>{
        self.data.clone()
    }

    fn get(&self) -> Result<Value, Error>{
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

    pub fn get_icon_and_description<'b>(code: i32) -> (&'b str, &'b str){
        match code{
            0            => ("☀️",  "Clear sky"),
            1 | 2 | 3    => ("⛅", "Mainly clear, partly cloudy, and overcast"),
            45 | 48      => ("🌁", "Fog and depositing rime fog"),
            51 | 53 | 55 => ("🌧️", "Drizzle: Light, moderate, and dense intensity"),
            56 | 57      => ("🌧️", "Freezing Drizzle: Light and dense intensity"),
            61 | 63 | 65 => ("🌧️", "Rain: Slight, moderate and heavy intensity"),
            66 | 67      => ("🌧️", "Freezing Rain: Light and heavy intensity"),
            71 | 73 | 75 => ("🌨️", "Snow fall: Slight, moderate, and heavy intensity"),
            77           => ("🌨️", "Snow grains"),
            80 | 81 | 82 => ("🌨️", "Rain showers: Slight, moderate, and violent"),
            85 | 86      => ("🌨️", "Snow showers slight and heavy"),
            95           => ("🌩️", "Thunderstorm: Slight or moderate"),
            96 | 99      => ("🌩️", "Thunderstorm with slight and heavy hail"),
            _            => ("🤷", "Not Available"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::openmeteo::OpenMeteoClient;

    #[test]
    fn test_openmeteo() {
        let omc = OpenMeteoClient::new("39.36667", "-0.41667", "Europe/Madrid");
        let data = omc.get();
        println!("{:?}", data);
        assert_eq!(data.is_ok(), true);
    }
}
