use serde::Deserialize;

use crate::weather::Weather;

#[derive(Deserialize)]
pub struct WeatherResponse {
    main: TemperatureData,
    weather: Vec<WeatherCondition>,
}

#[derive(Deserialize)]
struct TemperatureData {
    temp_max: f64,
    temp_min: f64,
}

#[derive(Deserialize)]
struct WeatherCondition {
    main: String,
}

impl WeatherResponse {
    pub fn into_weather(self) -> Weather {
        let condition = self.weather.into_iter().next().unwrap().main;
        Weather::new(
            self.main.temp_max.round() as i16,
            self.main.temp_min.round() as i16,
            condition,
        )
    }
}
