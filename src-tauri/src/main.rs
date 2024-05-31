// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Name {
    name: String,
}

#[tauri::command]
fn insert(name: &str) -> Result<(), ()> {
    insert_name(name).unwrap();
    Ok(())
}

fn insert_name(name: &str) -> Result<()> {
    let desktop_dir = dirs::desktop_dir().unwrap().to_str().unwrap().to_string() + "/database.db";
    let conn = Connection::open(desktop_dir)?;
    let name = Name {
        name: name.to_string(),
    };

    conn.execute(
        "INSERT INTO names (name) VALUES (?1)",
        [&name.name],
    )?;
    Ok(())
}

#[tauri::command]
fn select() -> Vec<String> {
    let mut name_vec = Vec::new();
    name_vec.push("lol".to_string());
    name_vec.push("βαρεθηκα να φτιαξω το select".to_string());
    name_vec.push("to insert δοευλευει ομως".to_string());
    name_vec
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select, insert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
