// This file will print the output from shell to the ratatui 
use std::thread;
use std::sync::{Arc , Mutex};

fn output_shell(
    mut reader: Box<dyn Read + Send>, // the dyn keyword is to tell that it will be complied at realtime 
    output: Arc<Mutex<String>>,
) {

    // Spawing a new thread for each output .
    //  we need to spawn a thread because we need to read the output from the shell and print it to the terminal and we don't want to block the main thread while reading the output from the shell
    thread::spawn(move || {
        let mut buffer = [0; 1024]; //buffer to store the output from the shell

        loop {
        }

    })
}