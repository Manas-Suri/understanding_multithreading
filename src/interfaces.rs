use std::sync::mpsc;
use std::thread;
use std::io;


pub fn run_interfaces(tx1: mpsc::Sender<String>) -> thread::JoinHandle<()> {
    thread::spawn(move ||{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();
        
        tx1.send(input).unwrap();
    })
}