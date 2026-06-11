// read and then write to the shell
use portable_pty::MasterPty;
use anyhow::Result;

pub fn system_io(
    master: &dyn MasterPty
) -> Result<(
    Box<dyn std::io::Read + Send>,
    Box<dyn std::io::Write + Send>,
    )> {
    let reader = master.try_clone_reader()?;
    let writer = master.take_writer()?;

    Ok((reader, writer))
}