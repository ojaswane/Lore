// used to find the exit code of the process

use std::process::Command;

pub fn get_exit_code(command: &str) -> Option<i32> {
    let output = Command::new("sh").arg("-c").arg(command).output().ok()?;

    Some(output.status.code().unwrap_or(-1))
}
