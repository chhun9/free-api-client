use crate::domain::AppData;
use std::fs;
use std::path::PathBuf;
use log::debug;

fn get_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    home_dir.join("dist")
}

pub fn get_file_read() -> PathBuf {
    get_file_path().join("api.json")
}

pub fn ensure_file_with_data() -> Result<PathBuf, String> {
    let app_dir = get_file_path();

    if !app_dir.exists() {
        fs::create_dir(&app_dir).map_err(|e| format!("Failed to create dist directory: {}", e))?;
    }

    let file_path = get_file_read();

    if !file_path.exists() {
        let empty_data = AppData::new();
        let _ = write_app_data_file(empty_data)?;
    }

    Ok(file_path)
}

pub fn read_file_content() -> Result<AppData, String> {
    let file_path = get_file_read();
    let content = fs::read(&file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    parse_content(content)
}

fn parse_content(content: Vec<u8>) -> Result<AppData, String> {
    let content_str = String::from_utf8(content).map_err(|e|
        format!("Failed to parse content as UTF-8: {}", e)
    )?;
    serde_json::from_str(&content_str).map_err(|e| format!("Failed to deserialize content: {}", e))
}

pub fn write_app_data_file(data: AppData) -> Result<(), String> {
    let file_path = get_file_read();
    let content = serde_json
        ::to_string_pretty(&data)
        .map_err(|e| format!("Failed to serialize data: {}", e))?;
    fs::write(file_path, content).map_err(|e| format!("Failed to write file: {}", e))
}
