// we will be using an library called protable-pty for the pesudo terminal 
use::portable_pty::{CommandBuilder, NativePtySystem , PtySize , Child};
use::color_eyre::eyre::Result; // this is the error handling library we will be using


pub fn shell() -> Result<()> {
    // Use the native pty implementation for the system
    let pty_system = NativePtySystem::default()?;

    // Create a new pty with default size
    let mut pty = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // spawinging the shell
    let cmd = pty.spawn_command(CommandBuilder::new("zsh"))?;
    
    let mut reader = pty.try_clone_reader()?;
    let mut writer = pty.try_clone_writer()?;

    
    Ok((pty, cmd))
    
}