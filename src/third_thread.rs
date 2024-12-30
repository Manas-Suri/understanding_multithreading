use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::led_struct::Led;


pub fn run_third_thread(tx: mpsc::Sender<String>,user: Arc<Mutex<Led>>) -> thread::JoinHandle<()> {
    thread::spawn(move ||{

        let led = user.lock().unwrap();
        println!("First thread toggled LED: state = {}, pin = {}", led.get_state(), led.get_pin());
        let val = String::from("Message from the third thread");
        println!("Running first thread!");
        tx.send(val).unwrap();
    })
}