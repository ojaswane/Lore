// we will be using a library called portable-pty for the pseudo terminal
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, Child, MasterPty , PtySystem };
use anyhow::Result;
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};


// the dyn keyword is used to indicate that the type of the master pty is not known at compile time and it will be determined at runtime
pub fn shell() -> Result<(Box<dyn MasterPty>, Box<dyn Child>)> { // Box means it will be using heap and dyn for runtime compiler
    // Use the native pty implementation for the system
    let pty_system = NativePtySystem::default();

    // Create a new pty with default size
    let pty = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // spawning the shell
    // this is the slave , it is basically the endpoint to the kernal 
    let cmd = pty.slave.spawn_command(CommandBuilder::new("zsh"))?;  
    // "Box" is basically to allocate the data into heap rather than stack
    
    // master , which is the endpoint to the user and it will be used to read and write data to the shell
    let _reader = pty.master.try_clone_reader()?;
    let _writer = pty.master.take_writer()?;
    
    Ok((pty.master, cmd))
}
