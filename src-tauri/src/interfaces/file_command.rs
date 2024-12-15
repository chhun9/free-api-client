use crate::infrastructure::file_service;
use crate::domain::models::AppData;
use tauri::command;

#[command]
pub fn ensure_data_file() -> Result<(), String> {
    file_service::ensure_file_with_data()
}

#[command]
pub fn read_data() -> Result<AppData, String> {
    file_service::read_file_content()
}

#[command]
pub fn save_data(data: String) -> Result<(), String> {
    let app_data: AppData = serde_json
        ::from_str(&data)
        .map_err(|e| format!("Failed to parse input data: {}", e))?;
    file_service::write_app_data_file(app_data)
}
