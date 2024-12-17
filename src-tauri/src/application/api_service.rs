use crate::domain::{ AppData, Collection, Api };
use crate::infrastructure::file_service;
use std::fs;
use log::debug;

fn read_app_data() -> Result<AppData, String> {
    let file_path = file_service::ensure_file_with_data()?;
    let read_content = fs::read(&file_path).map_err(|e| e.to_string())?;
    let read_content_str = String::from_utf8(read_content).map_err(|e| e.to_string())?;
    serde_json::from_str(&read_content_str).map_err(|e| e.to_string())
}

fn write_app_data(app_data: &AppData) -> Result<(), String> {
    let file_path = file_service::ensure_file_with_data()?;
    let updated_content = serde_json::to_string_pretty(app_data).map_err(|e| e.to_string())?;
    fs::write(&file_path, updated_content).map_err(|e| format!("Failed to write data: {}", e))
}

pub fn save_new_collection(data: String) -> Result<String, String> {
    let mut app_data = read_app_data()?;
    let mut new_collection: Collection = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    let ret_new_id: String = new_collection.new_id();
    app_data.collections_mut().push(new_collection);
    write_app_data(&app_data);
    Ok(ret_new_id)
}

pub fn save_new_api(data: String, collection_id: String) -> Result<String, String> {
    let mut app_data = read_app_data()?;
    let mut new_api: Api = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    let ret_new_id: String = new_api.new_id();

    if let Some(collection) = app_data.find_collection(&collection_id) {
        collection.apis_mut().push(new_api);
    } else {
        app_data.apis_mut().push(new_api);
    }
    write_app_data(&app_data);
    Ok(ret_new_id)
}

pub fn save_api(data: String) -> Result<(), String> {
    let mut app_data = read_app_data()?;
    let new_api: Api = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    if let Some(api_in_collection) = app_data.find_api_in_collection(new_api.id()) {
        *api_in_collection = new_api.clone();
    } else if let Some(api) = app_data.find_api(new_api.id()) {
        *api = new_api.clone();
    } else {
        return Err("API with the given ID not found.".to_string());
    }

    write_app_data(&app_data);
    Ok(())
}

pub fn delete_data_by_id(id: String) -> Result<(), String> {
    let mut app_data = read_app_data()?;

    app_data.collections_mut().retain(|collection| collection.id() != id);
    app_data.apis_mut().retain(|api| api.id() != id);
    for collection in app_data.collections_mut() {
        collection.apis_mut().retain(|api| api.id() != id);
    }

    write_app_data(&app_data);
    Ok(())
}

pub fn rename_data_by_id(id: String, new_name: String) -> Result<(), String> {
    let mut app_data = read_app_data()?;

    if let Some(collection) = app_data.find_collection(&id) {
        collection.new_name(new_name.clone());
    } else if let Some(api_in_collection) = app_data.find_api_in_collection(&id) {
        api_in_collection.new_name(new_name.clone());
    } else if let Some(api) = app_data.find_api(&id) {
        api.new_name(new_name.clone());
    } else {
        return Err("ID not found".to_string());
    }

    write_app_data(&app_data);
    Ok(())
}

pub fn duplicate_data_by_id(id: String) -> Result<(), String> {
    let mut app_data = read_app_data()?;
    let mut duplicated = false;

    if let Some(api) = app_data.find_api(&id) {
        let mut new_api = api.clone();
        new_api.new_id();
        app_data.apis_mut().push(new_api);
        duplicated = true;
    } else {
        for collection in app_data.collections_mut().iter_mut() {
            if collection.id() == id {
                let mut new_collection = collection.clone();
                new_collection.new_id();

                new_collection
                    .apis_mut()
                    .iter_mut()
                    .for_each(|api| {
                        api.new_id();
                    });

                app_data.collections_mut().push(new_collection);
                duplicated = true;
                break;
            } else {
                if
                    let Some(api) = collection
                        .apis_mut()
                        .iter()
                        .find(|api| api.id() == id)
                {
                    let mut new_api = api.clone();
                    new_api.new_id();

                    collection.apis_mut().push(new_api);
                    duplicated = true;
                    break;
                }
            }
        }
    }

    if !duplicated {
        return Err("ID not found".to_string());
    }

    write_app_data(&app_data);
    Ok(())
}
