use std::sync::{Arc, Weak};
use crate::{observable::Observable, observer::Observer};

pub struct StockMarket {
    pub observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    pub stock_price: f64,
}

impl StockMarket {
    pub fn new(stock_price: f64) -> Self {
        Self {
            observers: Vec::new(),
            stock_price
        }
    }

    pub fn set_stock_price(&mut self, stock_price: f64) {
        self.stock_price = stock_price;
        self.update();
    }

    pub fn stock_price(&self) -> f64 {
        self.stock_price
    }
}

impl Observable for StockMarket {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    
    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}