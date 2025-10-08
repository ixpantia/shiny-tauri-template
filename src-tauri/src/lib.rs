use std::{path::Path, sync::Mutex};
use tauri::{path::BaseDirectory, AppHandle, Manager, RunEvent, State};
use thiserror::Error;

#[derive(Debug, Error)]
enum CommandError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tauri Error: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("Path Error: Cannot convert path to string")]
    Path,
}

// we must manually implement serde::Serialize
impl serde::Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[cfg(target_os = "windows")]
fn run_app(
    app_state: State<AppState>,
    resource_path: &Path,
    handle: AppHandle,
    port: u16,
) -> Result<(), CommandError> {
     let path = handle.path().resolve("local-r/bin", BaseDirectory::Resource)?;

    let path_str = path.to_str().ok_or(CommandError::Path)?;
    let new_path = match std::env::var("PATH") {
        Ok(old_path) => format!("{};{}", path_str, old_path),
        Err(_) => path_str.to_string(),
    };

    let rscript_path = handle
        .path()
        .resolve("local-r/bin/R.exe", BaseDirectory::Resource)?;

    fn windows_to_unix_path(
        path:&str
    )-> String{
        path.replace('\\', "/")
    }

    let resource_path_str = windows_to_unix_path(resource_path.to_str().ok_or(CommandError::Path)?);

    // Kill existing process if any
    let mut child_process_lock = app_state.child_process.lock().unwrap();
    if let Some(mut child) = child_process_lock.take() {
        println!("Killing previous shiny app process");
        child.kill()?;
    }

    let child = std::process::Command::new(rscript_path)
        .arg("-e")
        .arg(format!(
            r#"options(shiny.port={port}); shiny::runApp('{}')"#,
            resource_path_str
        ))
        .current_dir(&resource_path)
        .env("PATH", new_path)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    *child_process_lock = Some(child);

    Ok(())
}


#[cfg(target_os = "macos")]
fn run_app(
    app_state: State<AppState>,
    resource_path: &Path,
    handle: AppHandle,
    port: u16,
) -> Result<(), CommandError> {
    let path = handle.path().resolve("local-r/", BaseDirectory::Resource)?;

    let path_str = path.to_str().ok_or(CommandError::Path)?;
    let new_path = match std::env::var("PATH") {
        Ok(old_path) => format!("{}:{}", path_str, old_path),
        Err(_) => path_str.to_string(),
    };

    let lib_path = handle
        .path()
        .resolve("local-r/lib", BaseDirectory::Resource)?;

    let lib_path_str = lib_path.to_str().ok_or(CommandError::Path)?;
    let new_ld_path = match std::env::var("LD_LIBRARY_PATH") {
        Ok(old_path) => format!("{}:{}", lib_path_str, old_path),
        Err(_) => lib_path_str.to_string(),
    };

    let new_dyld_path = match std::env::var("DYLD_LIBRARY_PATH") {
        Ok(old_path) => format!("{}:{}", lib_path_str, old_path),
        Err(_) => lib_path_str.to_string(),
    };

    let rscript_path = handle
        .path()
        .resolve("local-r/R", BaseDirectory::Resource)?;

    let rhome_path = handle.path().resolve("local-r/", BaseDirectory::Resource)?;

    let resource_path_str = resource_path.to_str().ok_or(CommandError::Path)?;

    // Kill existing process if any
    let mut child_process_lock = app_state.child_process.lock().unwrap();
    if let Some(mut child) = child_process_lock.take() {
        println!("Killing previous shiny app process");
        child.kill()?;
    }

    let child = std::process::Command::new(rscript_path)
        .arg("-e")
        .arg(format!(
            r#"options(shiny.port={port}); shiny::runApp('{}')"#,
            resource_path_str
        ))
        .current_dir(&resource_path)
        .env("R_HOME", &rhome_path)
        .env("PATH", new_path)
        .env("LD_LIBRARY_PATH", new_ld_path)
        .env("DYLD_LIBRARY_PATH", new_dyld_path)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    *child_process_lock = Some(child);

    Ok(())
}

#[derive(Default)]
struct AppState {
    child_process: Mutex<Option<std::process::Child>>,
}

#[tauri::command]
fn run_shiny_app(
    handle: tauri::AppHandle,
    app_state: State<AppState>,
) -> Result<String, CommandError> {
    let resource_path = handle.path().resolve("app/", BaseDirectory::Resource)?;

    let port: u16 = rand::random_range(3838..=4141);

    run_app(app_state, &resource_path, handle, port)?;

    let app_url = format!("http://localhost:{port}");
    println!("THE APP URL IS: {app_url}");
    Ok(app_url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_shiny_app])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle: &AppHandle, event: RunEvent| match event {
        RunEvent::Exit => {
            let app_state: State<AppState> = app_handle.state();
            if let Ok(mut child_lock) = app_state.child_process.lock() {
                if let Some(mut child) = child_lock.take() {
                    println!("Killing child process on exit");
                    if let Err(e) = child.kill() {
                        eprintln!("Failed to kill child process: {}", e);
                    }
                }
            };
        }
        _ => {}
    });
}
