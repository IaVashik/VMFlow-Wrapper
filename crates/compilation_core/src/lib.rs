use std::process::{Command, ExitStatus, Stdio};
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::Sender;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread;
use std::time::Duration;

mod builtin_commands;
use builtin_commands::process_built_in;
use vmflow_config_types::preset::Preset;
use vmflow_config_types::{GameConfiguration, VmfMap};

#[derive(Debug)]
pub enum ProcessingMessage {
    /// Обычное сообщение для лога
    LogInfo(String),
    /// Сообщение об успехе (можно выделить зеленым)
    LogSuccess(String),
    /// Предупреждение (можно выделить желтым/оранжевым)
    LogWarning(String),
    /// Ошибка (можно выделить красным)
    LogError(String),

    SetCurrentStepName(String),
    StepFinished,
    CompilationFinished,
    CompilationFailed(BackendError),
    AllStepsFinished,
}

#[derive(Debug)]
pub enum BackendError {
    IoError(io::Error),
    ProcessSpawnError(io::Error),
    ProcessWaitError(io::Error),
    CommandNotFound(String),
    Cancelled,
    StepFailed(ExitStatus),
    InvalidConfiguration,
    BuiltinFailed(String), 
}


#[derive(Debug)]
struct PlaceholderContext {

}


pub fn start_compilation_thread(tx: Sender<ProcessingMessage>, preset: Preset, game_config: GameConfiguration, maps: Vec<VmfMap>, is_cancelled: Arc<AtomicBool>) {
    thread::spawn(move || {
        // let PlaceholderContext ? // todo
        for map in &maps {
            let result = run_compilation(&preset, &game_config, map, tx.clone(), &is_cancelled);
    
            // Sending process result
            match result {
                Ok(_) => {
                    tx.send(ProcessingMessage::CompilationFinished).ok();
                }
                Err(e) => {
                    tx.send(ProcessingMessage::LogError(format!("Compilation failed: {:?}", e))).ok();
                    tx.send(ProcessingMessage::CompilationFailed(e)).ok();
                    return;
                }
            }
        }
    });
}

fn run_compilation(preset: &Preset,
    game_config: &GameConfiguration,
    map: &VmfMap,
    tx: Sender<ProcessingMessage>,
    is_cancelled: &Arc<AtomicBool>,
) -> Result<(), BackendError> {
    for compiler_step in &preset.apps {
        is_should_canceled(is_cancelled, &tx)?;
        if !compiler_step.activated || !map.activated {
            continue;
        }

        let step_name = compiler_step.name().to_string();
        tx.send(ProcessingMessage::SetCurrentStepName(step_name.clone())).ok();
        
        // Processing built-in command
        if compiler_step.config().is_builtin {
            process_built_in(step_name.as_str(), &tx);
            tx.send(ProcessingMessage::StepFinished).ok();
            continue;
        }

        // Processing compiler stuff 
        let mut executable = game_config.custom_apps_paths[compiler_step.compiler_idx].clone();
        if executable.is_empty() {
            return Err(BackendError::CommandNotFound(format!("Path for {step_name} not installed")));
        } 

        let mut command_args = compiler_step.get_command_params(); // todo process placeholders!1

        #[cfg(unix)]
        if executable.ends_with(".exe") {
            command_args.insert(0, executable);
            executable = "wine".to_string();
        }

        // resolve placeholders
        for arg in command_args.iter_mut() { // Получаем итератор изменяемых ссылок &mut String
            #[cfg(not(unix))]
            match arg.as_str() {
                "$game" => *arg = game_config.game_dir.to_string(),
                "$vmfFile" => *arg = map.path.to_string_lossy().to_string(),
                // "$binFolder" => *arg = bin_folder.to_string(),
                // TODO create full list of "templates"
                _ => {}
            }
            #[cfg(unix)]
            match arg.as_str() {
                "$game" => *arg = "Z:".to_string() + &game_config.game_dir,
                "$vmfFile" => *arg = "Z:".to_string() + &map.path.to_string_lossy(),
                // "$binFolder" => *arg = bin_folder.to_string(),
                // TODO create full list of "templates"
                _ => {}
            }
        }

        let work_dir = compiler_step.config().working_dir.as_ref();

        tx.send(ProcessingMessage::LogInfo(format!(
            "Executing: {executable} {command_args:?}"
        ))).ok();

        let mut command = Command::new(&executable);
        command.args(command_args);

        // Set Work Dir // TODO process this one
        // if let Some(wd) = work_dir {
        //     command.current_dir("/home/lavashik/.local/share/Steam/steamapps/common/Portal 2/bin"); // TODO!
        // }

        // Setting redirect stdout & stderr
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().map_err(|e| {
            eprintln!("ERROR: fking error: {e}");
            if e.kind() == io::ErrorKind::NotFound {
                BackendError::CommandNotFound(executable.to_string())
            } else {
                BackendError::ProcessSpawnError(e)
            }
        })?;

        // lets read stuff from stdout & stderr
        let stdout = child.stdout.take().expect("Stdout handle missing");
        let stderr = child.stderr.take().expect("Stderr handle missing");
        let tx_out = tx.clone();
        let tx_err = tx.clone();
        let cancel_out = is_cancelled.clone();
        let cancel_err = is_cancelled.clone();

        let stdout_thread = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line_result in reader.lines() {
                // Check for cancellation before processing the line
                if cancel_out.load(Ordering::SeqCst) { break; }
                match line_result {
                    Ok(text) => {
                        // TODO: potentially need to check for ASCII color, if it's a warning (yellow) - send as WarnInfo?!
                        tx_out.send(ProcessingMessage::LogInfo(text)).ok();
                    }
                    Err(e) => {
                        // TODO?: Should this be an error? Should we break the loop or ignore it?
                        // tx_err.send(ProcessingMessage::LogError(format!("stdout read error: {}", e))).ok();
                        // break;
                    }
                };
            }
        });
        let stderr_thread = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line_result in reader.lines() {
                if cancel_err.load(Ordering::SeqCst) { break; }
                match line_result {
                    Ok(text) => {
                        tx_err.send(ProcessingMessage::LogError(text)).ok();
                    }
                    Err(e) => {
                        // tx_err.send(ProcessingMessage::LogError(format!("stderr read error: {}", e))).ok();
                        // break;
                    }
                };
            }
        });
        
        loop {
            is_should_canceled(is_cancelled, &tx)?;
            match child.try_wait() {
                Ok(Some(status)) => { // Process Finished
                    stdout_thread.join().expect("stdout thread panicked");
                    stderr_thread.join().expect("stderr thread panicked");
                    if !status.success() {
                        return Err(BackendError::StepFailed(status));
                    }
                    break;
                },
                Ok(None) => { // The process is still running
                    thread::sleep(Duration::from_millis(50));
                }, 
                Err(e) => { // Error while waiting
                    stdout_thread.join().expect("stdout thread panicked");
                    stderr_thread.join().expect("stderr thread panicked");
                    return Err(BackendError::ProcessWaitError(e));
                } 
            }
        }
        
        tx.send(ProcessingMessage::StepFinished).ok();
    }
    Ok(())    
}

fn is_should_canceled(is_cancelled: &Arc<AtomicBool>, tx: &Sender<ProcessingMessage>) -> Result<(), BackendError> {
    if is_cancelled.load(Ordering::SeqCst) {
        // tx.send(ProcessingMessage::LogWarning("Compilation cancelled by user.".to_string())).ok();
        return Err(BackendError::Cancelled)
    }
    Ok(())
}


// fn resolve_placeholders(strint: &mut String) {}