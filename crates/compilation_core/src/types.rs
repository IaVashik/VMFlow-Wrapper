use std::{io, sync::Arc};

/// Represents errors that can occur during backend processing.
#[derive(Debug)]
pub enum BackendError {
    IoError(io::Error),
    ProcessSpawnError(io::Error),
    ProcessWaitError(io::Error),
    CommandNotFound(String),
    Cancelled,
    StepFailed(String, std::process::ExitStatus), // (name, status)
    // InvalidConfiguration,
    BuiltinFailed(String), // (name)
    Unknown,
}

type ProcessResult = Result<(), BackendError>;

#[derive(Debug)]
/// Represents events that occur during core processing.
pub enum CoreEvent {
    // BIG TODO!!!
    BatchStarted,

    MapStarted(usize, String),                  // (thread_id, map_name)
    StepStarted(usize, String, String),         // (thread_id, map_name, process_name)
    StepLog(usize, String, String, String),     // (thread_id, map_name, process_name)
    StepWarn(usize, String, String, String),    // (thread_id, map_name, process_name)
    StepErr(usize, String, String, String),     // (thread_id, map_name, process_name)

    StepFinished(usize, String, String),        // (thread_id, map_name, process_name)
    MapFinished(usize, String, ProcessResult),  // (thread_id, map_name, result)

    BatchCompleted(ProcessResult),              // (result)
    BatchCancelled,
    CancellationRequested,
}

/// Trait for handling job events.
pub trait JobEventHandler: Send + Sync {
    fn handle_event(&self, event: CoreEvent);
}

/// Sends or prints a core event based on the presence of an event handler.
pub fn send_or_print_event(event_handler: &Option<Arc<dyn JobEventHandler>>, event: CoreEvent) {
    match event_handler {
        Some(handler) => {
            handler.handle_event(event);
        }
        None => match event {
            CoreEvent::MapStarted(thread_id, map_name) => println!(
                "[EVENT] MapStarted(thread_id: {}, map_name: {})",
                thread_id, map_name
            ),
            CoreEvent::StepStarted(thread_id, map_name, process_name) => println!(
                "[EVENT] StepStarted(thread_id: {}, map_name: {}, process_name: {})",
                thread_id, map_name, process_name
            ),
            CoreEvent::StepLog(thread_id, map_name, process_name, log_message) => println!(
                "[EVENT] StepLog(thread_id: {}, map_name: {}, process_name: {}, log_message: {})",
                thread_id, map_name, process_name, log_message
            ),
            CoreEvent::StepFinished(thread_id, map_name, process_name) => println!(
                "[EVENT] StepFinished(thread_id: {}, map_name: {}, process_name: {})",
                thread_id, map_name, process_name
            ),
            CoreEvent::MapFinished(thread_id, map_name, result) => println!(
                "[EVENT] MapFinished(thread_id: {}, map_name: {}, result: {:?})",
                thread_id, map_name, result
            ),
            CoreEvent::BatchCompleted(result) => {
                println!("[EVENT] BatchCompleted(result: {:?})", result)
            },
            _ => println!("[EVENT] {:?}", event),
        },
    }
}
