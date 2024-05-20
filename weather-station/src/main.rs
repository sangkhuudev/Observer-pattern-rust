mod subject;
mod observer;
mod observable;

use std::sync::Arc;
use crate::observable::Observable;
use crate::observer::Observer;
use crate::subject::WeatherStation;

struct TemperatureDisplay {
    name: String,
}

impl TemperatureDisplay {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.into()
        })
    }
}

impl Observer for TemperatureDisplay {
    type Subject = WeatherStation;
    fn observe(&self, subject: &Self::Subject) {
        println!(
            "TemperatureDisplay {}: Current temperature is: {}",
            self.name, 
            subject.temperature()
        )
    }
}
fn main() {
    let mut weather_station = WeatherStation::new(25.0);
    let display1 = TemperatureDisplay::new("Display1");
    let display2 = TemperatureDisplay::new("Display2");

    weather_station.attach(display1.clone());
    weather_station.attach(display2.clone());
    weather_station.set_temperature(30.0);

}
