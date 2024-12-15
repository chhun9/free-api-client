use crate::domain::{ AppData, Api };
use crate::infrastructure::file_service;
use std::fs;

pub fn save_api(data: String) -> Result<(), String> {
    let read_content = fs::read(file_service::get_file_read()).map_err(|e| e.to_string())?;
    let read_content_str = String::from_utf8(read_content).map_err(|e| e.to_string())?;
    let mut app_data: AppData = serde_json::from_str(&read_content_str).map_err(|e| e.to_string())?;

    let new_api: Api = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let mut updated = false;

    for collection in app_data.collections_mut() {
        if
            let Some(existing_api) = collection
                .apis_mut()
                .iter_mut()
                .find(|api| api.id() == new_api.id())
        {
            *existing_api = new_api.clone();
            updated = true;
            break;
        }
    }

    if !updated {
        if
            let Some(existing_api) = app_data
                .apis_mut()
                .iter_mut()
                .find(|api| api.id() == new_api.id())
        {
            *existing_api = new_api.clone();
            updated = true;
        }
    }

    if !updated {
        return Err("API with the given ID not found.".to_string());
    }

    let updated_content = serde_json::to_string_pretty(&app_data).map_err(|e| e.to_string())?;
    fs
        ::write(file_service::get_file_read(), updated_content)
        .map_err(|e| format!("Failed to write data: {}", e))?;
    Ok(())
}

pub fn delete_data_by_id(id: String) -> Result<(), String> {
    let read_content = fs::read(file_service::get_file_read()).map_err(|e| e.to_string())?;
    let read_content_str = String::from_utf8(read_content).map_err(|e| e.to_string())?;
    let mut app_data: AppData = serde_json::from_str(&read_content_str).map_err(|e| e.to_string())?;

    app_data.collections_mut().retain(|collection| collection.id() != id);
    app_data.apis_mut().retain(|api| api.id() != id);
    for collection in app_data.collections_mut() {
        collection.apis_mut().retain(|api| api.id() != id);
    }

    let updated_content = serde_json::to_string_pretty(&app_data).map_err(|e| e.to_string())?;
    fs
        ::write(file_service::get_file_read(), updated_content)
        .map_err(|e| format!("Failed to write data: {}", e))?;
    Ok(())
}
