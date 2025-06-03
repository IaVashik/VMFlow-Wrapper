use std::{io::Error, sync::mpsc::Sender};
use crate::types::BackendError;

pub async fn process(step_name: &str) -> Result<(), BackendError> {
    match step_name {
        "COPY" => {eprintln!("Built-in func not implemented: COPY.")},
        "SHUTDOWN" => {system_shutdown();},
        _ => {
            return Err(BackendError::BuiltinFailed("Process Not Found".to_string())); // todo?
        }
    }
    Ok(())
}

fn system_shutdown() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        execute_command("shutdown", &["/s", "/f", "/t", "0"])
            .map_err(|e| format!("Windows shutdown command failed: {}", e))
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        execute_command("shutdown", &["-h", "now"])
            .map_err(|e| format!("Unix shutdown command failed: {}", e))
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err(String::from("Shutdown is not supported on this operating system with this program."))
    }
}

fn execute_command(program: &str, args: &[&str]) -> Result<(), Error> {
    let status = std::process::Command::new(program)
        .args(args)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{} {:?}' failed with status: {}", program, args, status),
        ))
    }
}
