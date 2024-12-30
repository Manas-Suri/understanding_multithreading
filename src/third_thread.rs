use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::led_struct::Led;


pub fn run_third_thread(tx: mpsc::Sender<String>,user: Arc<Mutex<Led>>) -> thread::JoinHandle<()> {
    thread::spawn(move ||{

        let mut led = user.lock().unwrap();
        // led.toggle(); // Toggle the LED state
        println!("First thread toggled LED: state = {}, pin = {}", led.get_state(), led.get_pin());


        let val = String::from("Message from the third thread");
        println!("Running first thread!");
        tx.send(val).unwrap();
    })
    // println!("Running1 first thread!");
    // Add your thread logic here
}