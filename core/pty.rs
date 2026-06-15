// we will be using a library called portable-pty for the pseudo terminal
use anyhow::Result;
use portable_pty::{Child, CommandBuilder, MasterPty, NativePtySystem, PtySize, PtySystem};

// the dyn keyword is used to indicate that the type of the master pty is not known at compile time and it will be determined at runtime
pub fn shell() -> Result<(Box<dyn MasterPty>, Box<dyn Child>)> {
    // Box means it will be using heap and dyn for runtime compiler
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
    let cmd = pty.slave.spawn_command(CommandBuilder::new("bash"))?;
    // "Box" is basically to allocate the data into heap rather than stack

    Ok((pty.master, cmd))
}
