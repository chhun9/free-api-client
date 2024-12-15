use super::*;

#[tauri::command]
pub fn save_api(data: String) -> Result<(), String> {
    api_service::save_api(data)
}
#[tauri::command]
pub fn delete_data(id: String) -> Result<(), String> {
    api_service::delete_data_by_id(id)
}