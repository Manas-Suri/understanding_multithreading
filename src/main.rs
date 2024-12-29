use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

mod first_thread;
mod second_thread;
mod third_thread;
mod led_struct;

fn main() {
    let (tx, rx) = mpsc::channel();
    let user = Arc::new(Mutex::new(led_struct::Led::new(false,15))); 
    
    let rx = Arc::new(Mutex::new(rx)); // Wrap rx in Arc<Mutex> for shared ownership

    loop {
        println!("Press '1' to start the first thread, '2' to start the second thread, or 'q' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "1" => {
                let first_thread_handle = first_thread::run_first_thread(tx.clone(),user.clone());
                first_thread_handle.join().unwrap();
                let second_thread_handle = second_thread::run_second_thread(Arc::clone(&rx)); // Pass clone of Arc
                second_thread_handle.join().unwrap();
            }
            "2" => {
                let third_thread_handle = third_thread::run_third_thread(tx.clone(),user.clone());
                third_thread_handle.join().unwrap();
                let second_thread_handle = second_thread::run_second_thread(Arc::clone(&rx)); // Pass clone of Arc
                second_thread_handle.join().unwrap();
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}
