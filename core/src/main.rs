#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri::command;

#[command]
async fn filesystemcall(argument: String) -> Result<String, String> {
    fs::create_dir("TEST_DIR").expect("Failed to create directory");
    println!("lololol {}", argument);
    Ok(argument.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![filesystemcall])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
