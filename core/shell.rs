// we will be using a library called portable-pty for the pseudo terminal
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, Child, MasterPty};
use color_eyre::eyre::Result; // this is the error handling library we will be using


pub fn shell() -> Result<(Box<dyn portable_pty::MasterPty>, Child)> {
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
    let cmd = pty.spawn_command(CommandBuilder::new("zsh"))?;
    
    let _reader = pty.try_clone_reader()?;
    let _writer = pty.try_clone_writer()?;

    Ok((pty.master, cmd))
}