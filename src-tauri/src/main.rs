#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tauri::Manager;
use crate::domain::AppState;

mod domain;
mod application;
mod infrastructure;
mod interfaces;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn main() {
    interfaces::file_command::ensure_data_file();
    tauri::Builder
        ::default()
        .setup(|app| {
            let app_state = Arc::new(TokioMutex::new(AppState::default()));
            app.manage(app_state);

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(
            tauri::generate_handler![
                interfaces::util_command::get_http_headers,
                interfaces::util_command::get_http_methods,
                interfaces::util_command::get_http_parameters,
                interfaces::file_command::save_data,
                interfaces::file_command::read_data,
                interfaces::api_command::save_api,
                interfaces::api_command::delete_data,
                interfaces::api_command::save_new_collection,
                interfaces::api_command::save_new_api,
                interfaces::api_command::rename_data,
                interfaces::api_command::duplicate_data,
                interfaces::request_command::cancel_request,
                interfaces::request_command::send_request
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
