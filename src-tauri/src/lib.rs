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

    let rscript_path = "/usr/local/bin/Rscript";

    let child_process = std::process::Command::new(rscript_path)
        .arg("-e")
        .arg(format!(
            "shiny::runApp('{}', port = {port})",
            resource_path.to_str().unwrap()
        ))
        .spawn();

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
