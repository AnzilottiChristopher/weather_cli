pub struct Weather {
    high_t: i16,
    low_t: i16,
    weather_conditions: String,
}

impl Weather {
    pub fn new(high: i16, low: i16, condition: String) -> Self {
        Self {
            high_t: high,
            low_t: low,
            weather_conditions: condition,
        }
    }

    // pub fn set_high_temp(&mut self, temp: i16) {
    //     self.high_t = temp;
    // }
    //
    // pub fn set_low_temp(&mut self, temp: i16) {
    //     self.low_t = temp;
    // }
    //
    // pub fn set_weather_conditions(&mut self, condition: String) {
    //     self.weather_conditions = condition;
    // }

    pub fn get_high_temp(&self) -> &i16 {
        &self.high_t
    }

    pub fn get_low_temp(&self) -> &i16 {
        &self.low_t
    }

    pub fn get_weather_conditions(&self) -> &str {
        &self.weather_conditions
    }
}
