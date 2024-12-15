use crate::domain::enums::{ HttpHeader, HttpMethod, HttpParameter };

#[tauri::command]
pub fn get_http_headers() -> Result<Vec<String>, String> {
    Ok(HttpHeader::get_all_headers())
}
#[tauri::command]
pub fn get_http_methods() -> Result<Vec<String>, String> {
    Ok(HttpMethod::get_all_methods())
}
#[tauri::command]
pub fn get_http_parameters() -> Result<Vec<String>, String> {
    Ok(HttpParameter::get_all_parameters())
}
