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
async fn open_file(app: tauri::AppHandle) -> Result<Option<FileResult>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("Word Documents", &["docx"])
        .blocking_pick_file();

    match file_path {
        Some(fp) => {
            let path_str = fp.to_string();
            let bytes = std::fs::read(&path_str).map_err(|e| e.to_string())?;
            Ok(Some(FileResult {
                path: path_str,
                bytes,
            }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
async fn save_file(path: String, data: Vec<u8>) -> Result<(), String> {
    std::fs::write(&path, &data).map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_file_dialog(app: tauri::AppHandle, data: Vec<u8>) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("Word Documents", &["docx"])
        .set_file_name("document.docx")
        .blocking_save_file();

    match file_path {
        Some(fp) => {
            let path_str = fp.to_string();
            std::fs::write(&path_str, &data).map_err(|e| e.to_string())?;
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![open_file, save_file, save_file_dialog])
        .run(tauri::generate_context!())
        .expect("error while running SofDocs desktop");
}
