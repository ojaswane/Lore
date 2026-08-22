// This file will print the output from shell to the ratatui
use std::io::Read;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
// use vt100::Parser;

const EXIT_MARKER_PREFIX: &str = "\x1b]777;LoreExit:";
const EXIT_MARKER_SUFFIX: char = '\x07';

pub fn output_shell(
    mut reader: Box<dyn Read + Send>, // the dyn keyword is to tell that it will be complied at realtime
    parser: Arc<Mutex<vt100::Parser>>,
    exit_code_sender: Sender<i32>,
) -> thread::JoinHandle<()> {
    // Spawing a new thread for each output .
    //  we need to spawn a thread because we need to read the output from the shell and print it to the terminal and we don't want to block the main thread while reading the output from the shell
    thread::spawn(move || {
        //The move keyword tells Rust to rip the variables out of the current function and give full ownership to a completely new background thread.
        let mut buffer = [0; 1024]; //buffer to store the output from the shell
        let mut pending_text = String::new();

        loop {
            match reader.read(&mut buffer) {
                // read the output from the shell and store it in the buffer
                Ok(n) if n > 0 => {
                    let mut parser_lock = parser.lock().unwrap(); // locks the variable for one process at a time 
                    parser_lock.process(&buffer[..n]);
                    drop(parser_lock);

                    let text = String::from_utf8_lossy(&buffer[..n]);
                    pending_text.push_str(&text);

                    while let Some(start) = pending_text.find(EXIT_MARKER_PREFIX) {
                        let code_start = start + EXIT_MARKER_PREFIX.len();

                        if let Some(end_offset) = pending_text[code_start..].find(EXIT_MARKER_SUFFIX)
                        {
                            let end = code_start + end_offset;
                            if let Ok(exit_code) = pending_text[code_start..end].parse::<i32>() {
                                let _ = exit_code_sender.send(exit_code);
                            }

                            pending_text.drain(..=end);
                        } else {
                            pending_text.drain(..start);
                            break;
                        }
                    }

                    if pending_text.len() > EXIT_MARKER_PREFIX.len() * 2 {
                        let keep_from = pending_text
                            .len()
                            .saturating_sub(EXIT_MARKER_PREFIX.len());
                        pending_text.drain(..keep_from);
                    }

                }

                _ => {
                    break;
                }
            }
        }
    })
}
