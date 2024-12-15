use super::*;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tauri::State;

use crate::domain::{ AppState, Header, HttpMethod };

#[tauri::command]
pub async fn cancel_request(state: State<'_, Arc<TokioMutex<AppState>>>) -> Result<(), String> {
    request_service::cancel_request(state.inner().clone()).await
}

#[tauri::command]
pub async fn send_request(
    state: State<'_, Arc<TokioMutex<AppState>>>,
    method: String,
    url: String,
    headers: Vec<Header>,
    body: Option<String>
) -> Result<serde_json::Value, String> {
    request_service::send_request(
        state.inner().clone(),
        HttpMethod::from_str(&method),
        url,
        headers,
        body
    ).await
}
