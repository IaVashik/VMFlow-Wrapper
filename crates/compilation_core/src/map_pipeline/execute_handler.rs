use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use async_process::{Command, Stdio};
use async_std::{io::{BufReadExt, BufReader}, stream::StreamExt, task::{self, JoinHandle}};
use crate::{send_or_print_event, types::BackendError, CoreEvent, JobEventHandler};

pub async fn execute_process(
    map_id: usize,
    map_name: String,
    step_name: String,
    //
    executable: String,
    arguments: Vec<String>,
    work_dir: String,
    cancel_flag: Arc<AtomicBool>,
    event_handler: Option<Arc<dyn JobEventHandler>>
) -> Result<(), BackendError> {
    let mut command = Command::new(&executable);
    command.args(arguments);
    if !work_dir.is_empty() {
        command.current_dir(work_dir);
    }

    // Redirect stdout and stderr so we can read them
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|e| {
        eprintln!("ERROR: spawn error: {e}");
        if e.kind() == futures::io::ErrorKind::NotFound {
            BackendError::CommandNotFound(executable)
        } else {
            BackendError::ProcessSpawnError(e)
        }
    })?;

    let child_stdout = child.stdout.take().expect("Stdout handle missing");
    let child_stderr = child.stderr.take().expect("Stderr handle missing");

    let stdout_task = spawn_stream_processing_task(
        child_stdout,
        map_id,
        map_name.clone(),
        step_name.clone(),
        Arc::clone(&cancel_flag),
        event_handler.clone(),
        false
    );

    let stderr_task = spawn_stream_processing_task(
        child_stderr,
        map_id,
        map_name.clone(),
        step_name.clone(),
        Arc::clone(&cancel_flag),
        event_handler.clone(),
        true
    );

    let mut status_result: Result<std::process::ExitStatus, BackendError> = Err(BackendError::Unknown);

    loop {
        if cancel_flag.load(Ordering::Relaxed) {
            if let Err(e) = child.kill() { // Пытаемся убить процесс
                println!("ERROR: Failed to kill process on cancellation: {}", e)
            }
            status_result = Err(BackendError::Cancelled);
            break;
        }
        match child.try_status() {
            Ok(Some(status)) => { // Process Finished
                status_result = Ok(status);
                break;
            },
            Ok(None) => { // The process is still running
                task::sleep(std::time::Duration::from_millis(100)).await;
            },
            Err(e) => { // Error while waiting
                status_result = Err(BackendError::ProcessWaitError(e));
                break;
            }
        }
    }

    // Wait for the stdout/stderr reading tasks to complete.
    stdout_task.await;
    stderr_task.await;

    // Returns the result based on the status
    match status_result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(BackendError::StepFailed(step_name, status)),
        Err(e_msg) => Err(e_msg),
    }
}


/// Helper function to spawn an asynchronous task for reading lines from a stream.
fn spawn_stream_processing_task<S>(
    stream_source: S,
    map_id: usize,
    map_name: String,
    step_name: String,
    cancel_flag: Arc<AtomicBool>,
    event_handler: Option<Arc<dyn JobEventHandler>>,
    is_err: bool,
) -> JoinHandle<()>
where
    S: async_std::io::Read + Unpin + Send + 'static,
{
    task::spawn(async move {
        let mut lines = BufReader::new(stream_source).lines();

        while let Some(line_result) = lines.next().await {
            if cancel_flag.load(Ordering::Relaxed) { break }

            match line_result {
                Ok(raw_line) => {
                    let event = if is_err {
                        CoreEvent::StepErr(map_id, map_name.clone(), step_name.clone(), raw_line)
                    }
                    else {
                        CoreEvent::StepLog(map_id, map_name.clone(), step_name.clone(), raw_line)
                    };
                    send_or_print_event(&event_handler, event);
                }
                Err(e) => {}
            }
        }
    })
}
