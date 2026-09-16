use std::marker::PhantomData;

pub struct Disconnected;
pub struct Connecting;
pub struct Ready;

pub struct Session<S> {
    url: String,
    _state: PhantomData<S>,
}

impl Session<Disconnected> {
    pub fn new(url: &str) -> Self {
        Session::<Disconnected> {
            url: String::from(url),
            _state: PhantomData,
        }
    }

    pub fn connect(self) -> Session<Connecting> {
        Session::<Connecting> {
            url: self.url,
            _state: PhantomData,
        }
    }
}

impl Session<Connecting> {
    pub fn authenticate(self) -> Session<Ready> {
        Session::<Ready> {
            url: self.url,
            _state: PhantomData,
        }
    }
}

impl Session<Ready> {
    pub fn disconnect(self) -> Session<Disconnected> {
        Session::<Disconnected> {
            url: self.url,
            _state: PhantomData,
        }
    }

    pub fn publish(&mut self, data: &str) {
        println!("publishing data {}", data);
    }
}
