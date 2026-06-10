// read and then write to the shell
use portable_pty::MasterPty;
use anyhow::Result;

pub fn read_and_write(
    master: &dyn MasterPty,
) -> Result<()> {

    let reader = master.try_clone_reader()?;
    let writer = master.take_writer()?;

    Ok(())
}