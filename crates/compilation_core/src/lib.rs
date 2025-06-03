use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use async_lock::Semaphore;
use async_std::task;

mod map_pipeline;
pub use map_pipeline::process_map_async;

mod types;
pub use types::{CoreEvent, JobEventHandler, send_or_print_event};

use crate::types::BackendError;

#[derive(Default, Clone)]
pub struct CompilationSessionSettings {
    pub preset: vmflow_config_types::preset::Preset,
    pub game_config: vmflow_config_types::GameConfiguration,
}

pub struct CompilationSession {
    settings: Arc<CompilationSessionSettings>,
    cancel_flag: Arc<AtomicBool>,
    max_threads: usize,
    event_handler: Option<Arc<dyn JobEventHandler>>,
}

impl Default for CompilationSession {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            cancel_flag: Default::default(),
            max_threads: 1,
            event_handler: None,
        }
    }
}

impl CompilationSession {
    pub fn new(
        preset: vmflow_config_types::preset::Preset,
        game_config: vmflow_config_types::GameConfiguration,
        max_threads: usize,
        event_handler: Option<Arc<dyn JobEventHandler>>,
    ) -> Self {
        let settings = CompilationSessionSettings {
            preset,
            game_config,
        };
        Self {
            settings: Arc::new(settings),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            event_handler,
            max_threads,
        }
    }

    /// Cancels the current batch processing job.
    pub fn cancel_batch(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        send_or_print_event(&self.event_handler, CoreEvent::CancellationRequested);
    }

    /// Starts a batch processing job, processing maps concurrently using async tasks.
    /// Returns a JoinHandle for the spawned thread, providing a way to await completion and retrieve results.
    pub fn start_batch(
        &self,
        maps_to_process: Vec<vmflow_config_types::VmfMap>,
    ) -> thread::JoinHandle<Result<Vec<Result<(), BackendError>>, BackendError>> {
        let settings_for_thread = Arc::clone(&self.settings);
        let cancel_flag_for_thread = Arc::clone(&self.cancel_flag);
        let event_handler_for_thread = self.event_handler.as_ref().map(Arc::clone);
        let max_threads: usize = self.max_threads;

        thread::spawn(move || {
            task::block_on(run_batch_concurrently(
                maps_to_process,
                settings_for_thread,
                cancel_flag_for_thread,
                event_handler_for_thread,
                max_threads,
            ))
        })
    }

    pub async fn start_batch_async(&self, maps_to_process: Vec<vmflow_config_types::VmfMap>) {
        let settings_for_thread = Arc::clone(&self.settings);
        let cancel_flag_for_thread = Arc::clone(&self.cancel_flag);
        let event_handler_for_thread = self.event_handler.as_ref().map(Arc::clone);

        run_batch_concurrently(
            maps_to_process,
            settings_for_thread,
            cancel_flag_for_thread,
            event_handler_for_thread,
            self.max_threads,
        )
        .await;
    }

    // submit_map?
}

/// Core asynchronous logic for a batch of maps
async fn run_batch_concurrently(
    maps_to_process: Vec<vmflow_config_types::VmfMap>,
    settings: Arc<CompilationSessionSettings>,
    cancel_flag: Arc<AtomicBool>,
    event_handler: Option<Arc<dyn JobEventHandler>>,
    max_concurrent_maps: usize,
) -> Result<Vec<Result<(), BackendError>>, BackendError> {
    send_or_print_event(&event_handler, CoreEvent::BatchStarted);

    let semaphore = Arc::new(Semaphore::new(max_concurrent_maps.max(1)));
    let mut task_handles = Vec::new();

    for map_info in maps_to_process {
        if cancel_flag.load(Ordering::Relaxed) { break }
        if !map_info.activated { continue }

        let map_settings = Arc::clone(&settings);
        let map_cancel_flag = Arc::clone(&cancel_flag);
        let map_event_handler = event_handler.as_ref().map(Arc::clone);
        let permit_semaphore = Arc::clone(&semaphore);

        let handle = task::spawn(async move {
            // Asynchronously wait for permission from the semaphore
            let _permit = permit_semaphore.acquire().await; // Just .acquire() for RAII guard in async

            process_map_async(map_info, map_settings, map_cancel_flag, map_event_handler).await
        });
        task_handles.push(handle);
    }

    // Wait for all spawned asynchronous tasks to complete
    let results_from_join = futures::future::join_all(task_handles).await;

    let final_map_results: Vec<Result<(), BackendError>> = results_from_join
        .into_iter()
        .map(|join_result| {
            println!("== Process status: {:?}", join_result);
            join_result // todo, add process result here
        })
        .collect();

    // Batch completion logic
    if cancel_flag.load(Ordering::Relaxed) {
        send_or_print_event(&event_handler, CoreEvent::BatchCancelled);
        return Err(BackendError::Cancelled);
    }
    // TODO!
    // if final_map_results.iter().any(|r| r.is_err()) {
    //     let err_msg = "One or more maps failed to compile.".to_string();
    //     send_or_print_event(&event_handler, CoreEvent::Log(Err(err_msg.clone())));
    //     send_or_print_event(&event_handler, CoreEvent::BatchCompleted(Error(())));
    //     return Err(err_msg);
    // }

    send_or_print_event(&event_handler, CoreEvent::BatchCompleted(Ok(())));
    println!("PROCESS FINISHED!");
    Ok(final_map_results)
}
