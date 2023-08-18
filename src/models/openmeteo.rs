use serde::Deserialize;
use serde_json::Value;
use anyhow::Error;
use chrono::{NaiveDate, NaiveDateTime};

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

    pub fn get_current_weather(&self) -> Option<CurrentWeather>{
        match self.data.clone(){
            Some(value) => {
                let cw = value.get("current_weather").unwrap().clone();
                Some(CurrentWeather::new(cw))
            },
            None => None,
        }
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
}

pub fn get_icon_and_description<'b>(code: i64) -> (&'b str, &'b str){
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
#[derive(Deserialize, Debug)]
pub struct DailyWeather{
    time: chrono::NaiveDate,
    temperature_min: f64,
    temperature_max: f64,
    weathercode: i64,
    sunrise: chrono::NaiveDateTime,
    sunset: chrono::NaiveDateTime,
    uv_index_max: f64,
    precipitation_probability_max: i64,
}
impl DailyWeather{
    pub fn new(time_str: String, weathercode: i64, temperature_min: f64,
            temperature_max: f64, sunrise_str: String, sunset_str: String,
            uv_index_max: f64, precipitation_probability_max: i64) -> Self{
        let time = NaiveDate::parse_from_str(&time_str, "%Y-%m-%d").unwrap();
        let sunrise = NaiveDateTime::parse_from_str(
            &sunrise_str, "%Y-%m-%dT%H:%M:%s").unwrap();
        let sunset = NaiveDateTime::parse_from_str(
            &sunset_str, "%Y-%m-%dT%H:%M:%s").unwrap();
        Self{
            time,
            temperature_min,
            temperature_max,
            weathercode,
            sunrise,
            sunset,
            uv_index_max,
            precipitation_probability_max,
        }

    }
}

#[derive(Debug)]
pub struct CurrentWeather{
    temperature: f64,
    windspeed: f64,
    winddirection: i64,
    weathercode: i64,
    is_day: i64,
}

impl CurrentWeather{
    pub fn new(value: Value) -> Self{
        let temperature = value.get("temperature").unwrap().as_f64().unwrap();
        let windspeed = value.get("windspeed").unwrap().as_f64().unwrap();
        let winddirection = value.get("winddirection").unwrap().as_i64().unwrap();
        let weathercode = value.get("weathercode").unwrap().as_i64().unwrap();
        let is_day = value.get("is_day").unwrap().as_i64().unwrap();
        Self{
            temperature,
            windspeed,
            winddirection,
            weathercode,
            is_day,
        }
    }
    pub fn get_temperature(&self) -> f64{
        self.temperature
    }
    pub fn get_winspeed(&self) -> f64{
        self.windspeed
    }
    pub fn get_winddirection(&self) -> i64{
        self.winddirection
    }
    pub fn is_day(&self) -> i64{
        self.is_day
    }
    pub fn get_icon_and_description<'b>(&self) -> (&'b str, &'b str){
        get_icon_and_description(self.weathercode)
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
    #[test]
    fn test_cw() {
        let mut omc = OpenMeteoClient::new("39.36667", "-0.41667", "Europe/Madrid");
        omc.refresh();
        println!("======================================");
        println!("{:?}", omc.data);
        println!("======================================");
        let data = omc.get_current_weather();
        println!("{:?}", data);
        let cw = data.unwrap();
        println!("{:?}", cw);
        assert_eq!(cw.is_day(), 1);
    }
}
