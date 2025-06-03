use std::{fmt::Arguments, sync::{
    atomic::{AtomicBool, Ordering}, Arc
}};

use vmflow_config_types::selected_compiler::SelectedCompiler;

use crate::{send_or_print_event, types::BackendError, CompilationSessionSettings, CoreEvent, JobEventHandler};

mod builtin_commands;
mod execute_handler;
pub use execute_handler::execute_process;

/// todo
pub async fn process_map_async(
    map_info: vmflow_config_types::VmfMap,
    settings: Arc<CompilationSessionSettings>,
    cancel_flag: Arc<AtomicBool>,
    event_fn: Option<Arc<dyn JobEventHandler>>,
) -> Result<(), BackendError> {
    send_or_print_event(&event_fn, CoreEvent::MapStarted(map_info.order_idx, map_info.name.clone()));

    for compiler_step in &settings.preset.apps {
        if cancel_flag.load(Ordering::Relaxed) { return Ok(()) }

        let step_name = compiler_step.name().to_string();
        send_or_print_event(&event_fn, CoreEvent::StepStarted(map_info.order_idx, map_info.name.clone(), step_name.clone()));

        // Processing built-in command
        if compiler_step.config().is_builtin {
            builtin_commands::process(step_name.as_str()).await?;
        }
        // Processing compiler stuff
        else {
            spawn_process(
                &map_info,
                &settings,
                compiler_step,
                &cancel_flag,
                event_fn.clone(), // todo правильно ли это? будет ли он клонировать arc?
            ).await?;
        }

        send_or_print_event(&event_fn, CoreEvent::StepFinished(map_info.order_idx, map_info.name.clone(), compiler_step.name().to_string()));
    }

    send_or_print_event(&event_fn, CoreEvent::MapFinished(map_info.order_idx, map_info.name.clone(), Ok(())));
    Ok(())
}

async fn spawn_process( // todo rename
    map_info: &vmflow_config_types::VmfMap,
    settings: &Arc<CompilationSessionSettings>,
    compiler: &SelectedCompiler,
    cancel_flag: &Arc<AtomicBool>,
    event_handler: Option<Arc<dyn JobEventHandler>>,
) -> Result<(), BackendError> {
    let mut executable = settings.game_config.custom_apps_paths[compiler.compiler_idx].clone();
    if executable.is_empty() {
        return Err(BackendError::CommandNotFound(format!(
            "Path for {} not installed",
            compiler.name()
        )));
    }

    let mut work_dir = compiler.config().working_dir.clone().unwrap_or("$binFolder".to_string());
    resolve_placeholders(&mut work_dir, map_info, settings);

    let mut command_args = compiler.get_command_params(); // todo process placeholders!
    command_args.iter_mut().for_each(|arg| resolve_placeholders(arg, map_info, settings));

    #[cfg(unix)]
    if executable.ends_with(".exe") {
        command_args.insert(0, executable);
        executable = "wine".to_string();
    }

    // bruh bruh bruh bruh todo
    send_or_print_event(&event_handler, CoreEvent::StepLog(
        map_info.order_idx,
        map_info.name.clone(),
        compiler.name().to_string(),
        format!("Executing: {executable} {command_args:?}")
    ));

    execute_process(
        map_info.order_idx,
        map_info.name.clone(),
        compiler.name().to_string(), // todo
        executable,
        command_args,
        work_dir,
        Arc::clone(cancel_flag),
        event_handler
    ).await?;

    Ok(())
}


pub fn resolve_placeholders(
    arg: &mut String,
    map_info: &vmflow_config_types::VmfMap,
    settings: &Arc<CompilationSessionSettings>
) {
    // #[cfg(not(unix))] // TODO PLACEHOLDER!

    #[cfg(unix)]
    match arg.as_str() { // todo what is there is no disk Z?
        "$gameDir" => *arg = "Z:".to_string() + &settings.game_config.game_dir,
        // "$gameName" => *arg = "Z:".to_string() + &settings.game_config.game_dir,
        // "$gameExe" => *arg = "Z:".to_string() + &settings.game_config.game_dir,

        // "$mapDir" => *arg = "Z:".to_string() + &map_info.path.to_string_lossy(),
        "$mapFile" => *arg = "Z:".to_string() + &map_info.path.to_string_lossy(),
        // "$mapName" => *arg = "Z:".to_string() + &map_info.path.to_string_lossy(),

        "$binFolder" => *arg = settings.game_config.bin_dir.clone(),
        "$outputDir" => *arg = settings.game_config.output_dir.clone(),

        // FUCKING TODO, because fucking hard-code
        "$vbsp" => *arg = settings.game_config.custom_apps_paths[0].clone(),
        "$vvis" => *arg = settings.game_config.custom_apps_paths[1].clone(),
        "$vrad" => *arg = settings.game_config.custom_apps_paths[2].clone(),
        "$bspZip" => *arg = settings.game_config.custom_apps_paths[3].clone(),
        // "vbspInfo" => *arg = settings.game_config.custom_apps_paths[4].clone(),

        _ => {}
    }
}
