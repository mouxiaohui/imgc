#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::color::get_image_color;

mod color;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_image_color])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
