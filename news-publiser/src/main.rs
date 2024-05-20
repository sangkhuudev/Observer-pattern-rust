use std::sync::Arc;

use observable::Observable;
use observer::Observer;
use subject::NewsPublisher;

mod observable;
mod observer;
mod subject;

struct NewsReader {
    name: String,
}

impl NewsReader {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.into() })
    }
}

impl Observer for NewsReader {
    type Subject = NewsPublisher;
    fn observe(&self, subject: &Self::Subject) {
        println!(
            "NewsReader {}: Breaking news - {}",
            self.name,
            subject.latest_news()
        );
    }
}

fn main() {
    let mut news_publisher = NewsPublisher::new("Initial News");

    let reader1 = NewsReader::new("Reader1");
    let reader2 = NewsReader::new("Reader2");

    news_publisher.attach(reader1.clone());
    news_publisher.attach(reader2.clone());

    news_publisher.set_latest_news("New event just happened!");
}
