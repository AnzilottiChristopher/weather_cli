mod weather;
mod weather_response;

use ::std::env;
use std::io::{Write, stdin, stdout};

use crate::{weather::Weather, weather_response::WeatherResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("OPENWEATHER_API").expect("API key not found");

    let url = "https://api.openweathermap.org/data/2.5/weather";

    println!("Welcome to weather app!");

    loop {
        print!("Please input your city: ");
        stdout().flush().unwrap();

        let mut city = String::new();
        stdin().read_line(&mut city).unwrap();

        let response = reqwest::Client::new()
            .get(url)
            .query(&[
                ("q", city.as_str()),
                ("units", "imperial"),
                ("appid", api_key.as_str()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            println!("City not found. Please try again");
            continue;
        }

        let weather_response: WeatherResponse = response.json().await?;

        let application: Weather = weather_response.into_weather();

        println!("{}", city);
        println!("High: {}", application.get_high_temp());
        println!("Low: {}", application.get_low_temp());
        println!("Condition: {}", application.get_weather_conditions());
        println!();
    }

    // Ok(())
}
