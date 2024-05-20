use std::sync::{Arc, Weak};
use crate::{observable::Observable, observer::Observer};

pub struct NewsPublisher {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    latest_news: String,
}

impl NewsPublisher {
    pub fn new(latest_news: &str) -> Self {
        Self {
            observers: Vec::new(),
            latest_news: latest_news.into(),
        }
    }

    pub fn latest_news(&self) -> &str {
        &self.latest_news
    }

    pub fn set_latest_news(&mut self, latest_news: &str) {
        self.latest_news = latest_news.into();
        self.update();
    }
}

impl Observable for NewsPublisher {
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
