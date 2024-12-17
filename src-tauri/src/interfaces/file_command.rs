use super::*;
use crate::domain::models::AppData;
use std::path::PathBuf;

#[tauri::command]
pub fn ensure_data_file() -> Result<PathBuf, String> {
    file_service::ensure_file_with_data()
}

#[tauri::command]
pub fn read_data() -> Result<AppData, String> {
    file_service::read_file_content()
}

#[tauri::command]
pub fn save_data(data: String) -> Result<(), String> {
    let app_data: AppData = serde_json
        ::from_str(&data)
        .map_err(|e| format!("Failed to parse input data: {}", e))?;
    file_service::write_app_data_file(app_data)
}
