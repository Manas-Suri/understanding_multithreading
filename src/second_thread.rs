use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn run_second_thread(rx: Arc<Mutex<mpsc::Receiver<String>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Running second thread!");

        match rx.lock() {
            Ok(locked_rx) => {
                match locked_rx.recv() {
                    Ok(received) => {
                        println!("Second thread received: {}", received);
                    }
                    Err(_) => {
                        eprintln!("Error: No value received from the channel.");
                        return; // Exit the thread early
                    }
                }
            }
            Err(_) => {
                eprintln!("Error: Failed to lock the receiver.");
                return; // Exit the thread early
            }
        }
        // Lock the receiver before accessing it
        // let received = rx.lock().unwrap().recv().unwrap();
        
        // println!("Second thread received: {}", received);
    })
}
