use std::sync::{Arc, Mutex};
// use std::thread;
// use std::sync::mpsc;

pub struct Led {
    state: bool,
    pin: i32,
}

impl Led {
    // Constructor for Led
    pub fn new(state: bool, pin: i32) -> Self {
        Led { state, pin }
    }

    // Example method to toggle state
    pub fn toggle(&mut self) {
        self.state = !self.state;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn get_pin(&self) -> i32 {
        self.pin
    }
}
