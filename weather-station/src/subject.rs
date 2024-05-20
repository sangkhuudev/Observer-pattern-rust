use std::sync::{Arc, Weak};
use crate::{observable::Observable, observer::Observer};
//Use Weak poiter when the oject goes out of scope we can ignore them instead of keeping them alive
// Use Arc to share ownership across threads
pub struct WeatherStation {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    temperature: f64,
}

impl WeatherStation {
    pub fn new(temperature: f64) -> Self {
        Self {
            observers: Vec::new(),
            temperature,
        }
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.update();
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

impl Observable for WeatherStation {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade()) // upgrade Weak to Arc
            .for_each(|o| o.observe(self))
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        //The ptr_eq method checks if the weak reference f points to the same allocation 
        //as the weak reference created by Arc::downgrade(&observer)
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer))); 
    }
}