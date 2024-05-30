// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn insert(name: &str) -> Result<String, String> {
    // TODO: actually insert name into database
    Ok("success".into())
}

#[tauri::command]
fn select() -> Vec<String> {
    let mut name_vec = Vec::new();
    // TODO: Read names from database
    // for now returning placeholder names:
    name_vec.push("val.exe".to_string());
    name_vec.push("sperdev".to_string());
    name_vec.push("blackassasin4".to_string());
    name_vec
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![insert])
        .invoke_handler(tauri::generate_handler![select])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
