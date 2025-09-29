use tauri::{path::BaseDirectory, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_shiny_app(handle: tauri::AppHandle) -> String {
    let resource_path = handle
        .path()
        .resolve("app/", BaseDirectory::Resource)
        .unwrap();

    let port: u16 = rand::random_range(3838..=4141);

    let path = handle
        .path()
        .resolve("local-r/", BaseDirectory::Resource)
        .unwrap();

    let new_path = match std::env::var("PATH") {
        Ok(old_path) => format!("{}:{}", path.to_str().unwrap(), old_path),
        Err(_) => path.to_str().unwrap().to_string(),
    };

    let lib_path = handle
        .path()
        .resolve("local-r/lib", BaseDirectory::Resource)
        .unwrap();

    let new_ld_path = match std::env::var("LD_LIBRARY_PATH") {
        Ok(old_path) => format!("{}:{}", lib_path.to_str().unwrap(), old_path),
        Err(_) => lib_path.to_str().unwrap().to_string(),
    };

    let new_dyld_path = match std::env::var("DYLD_LIBRARY_PATH") {
        Ok(old_path) => format!("{}:{}", lib_path.to_str().unwrap(), old_path),
        Err(_) => lib_path.to_str().unwrap().to_string(),
    };

    let rscript_path = handle
        .path()
        .resolve("local-r/R", BaseDirectory::Resource)
        .unwrap();

    let rhome_path = handle
        .path()
        .resolve("local-r/", BaseDirectory::Resource)
        .unwrap();

    let child_process = std::process::Command::new(rscript_path)
        .arg("-e")
        .arg(format!(
            r#"shiny::runApp('{}', port = {port})"#,
            resource_path.to_str().unwrap()
        ))
        .current_dir(resource_path)
        .env("R_HOME", rhome_path)
        .env("PATH", new_path)
        .env("LD_LIBRARY_PATH", new_ld_path)
        .env("DYLD_LIBRARY_PATH", new_dyld_path)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap();

    let app_url = format!("http://localhost:{port}");
    println!("THE APP URL IS: {app_url}");
    app_url
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_shiny_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
