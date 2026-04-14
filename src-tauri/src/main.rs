#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;

#[derive(Serialize)]
struct FileResult {
    path: String,
    bytes: Vec<u8>,
}

#[tauri::command]
async fn open_file() -> Result<Option<FileResult>, String> {
    Ok(None)
}

#[tauri::command]
async fn save_file(path: String, data: Vec<u8>) -> Result<(), String> {
    std::fs::write(&path, &data).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![open_file, save_file])
        .run(tauri::generate_context!())
        .expect("error while running SofDocs desktop");
}
