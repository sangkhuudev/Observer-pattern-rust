use std::sync::Arc;

use observable::Observable;
use observer::Observer;
use subject::StockMarket;

mod observable;
mod observer;
mod subject;

struct StockDisplay {
    name: String,
}

impl StockDisplay {
    fn new(name: &str) -> Arc<Self> {
        Arc::new( Self { name: name.into()})
    }
}

impl Observer for StockDisplay {
    type Subject = StockMarket;
    fn observe(&self, subject: &Self::Subject) {
        println!(
            "StockDisplay {}: Current stock price is {}",
            self.name,
            subject.stock_price()
        );
    }
}

fn main() {
    let mut stock_market = StockMarket::new(150.0);

    let display1 = StockDisplay::new("Display1");
    let display2 = StockDisplay::new("Display2");

    stock_market.attach(display1.clone());
    stock_market.attach(display2.clone());

    stock_market.set_stock_price(160.0);
}