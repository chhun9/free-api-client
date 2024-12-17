use super::*;

#[tauri::command]
pub fn save_api(data: String) -> Result<(), String> {
    api_service::save_api(data)
}
#[tauri::command]
pub fn delete_data(id: String) -> Result<(), String> {
    api_service::delete_data_by_id(id)
}
#[tauri::command]
pub fn save_new_collection(data: String) -> Result<String, String> {
    api_service::save_new_collection(data)
}
#[tauri::command]
pub fn save_new_api(data: String, collection_id: String) -> Result<String, String> {
    api_service::save_new_api(data, collection_id)
}
#[tauri::command]
pub fn rename_data(id: String, new_name: String) -> Result<(), String> {
    api_service::rename_data_by_id(id, new_name)
}
#[tauri::command]
pub fn duplicate_data(id: String) -> Result<(), String> {
    api_service::duplicate_data_by_id(id)
}
