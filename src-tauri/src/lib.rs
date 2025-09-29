use tauri::{path::BaseDirectory, Manager};
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

#[tauri::command]
fn run_shiny_app(handle: tauri::AppHandle) -> Result<String, CommandError> {
    let resource_path = handle.path().resolve("app/", BaseDirectory::Resource)?;

    let port: u16 = rand::random_range(3838..=4141);

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
    let _child_process = std::process::Command::new(rscript_path)
        .arg("-e")
        .arg(format!(
            r#"shiny::runApp('{}', port = {port})"#,
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

    let app_url = format!("http://localhost:{port}");
    println!("THE APP URL IS: {app_url}");
    Ok(app_url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![run_shiny_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
