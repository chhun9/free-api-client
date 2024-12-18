use reqwest::Client;
use tokio_util::sync::CancellationToken;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use log::debug;

use crate::domain::{ AppState, Header, HttpMethod, HttpResponse };

pub async fn send_request(
    state: Arc<TokioMutex<AppState>>,
    method: HttpMethod,
    url: String,
    headers: Vec<Header>,
    body: Option<String>
) -> Result<HttpResponse, String> {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    {
        let mut state = state.lock().await;
        state.cancellation_token = Some(token.clone());
    }

    let handle = tokio::spawn(async move {
        let client = Client::new();

        let mut req = match method {
            HttpMethod::GET => client.get(&url),
            HttpMethod::POST => client.post(&url),
            HttpMethod::PUT => client.put(&url),
            HttpMethod::PATCH => client.patch(&url),
            HttpMethod::DELETE => client.delete(&url),
        };

        for header in headers {
            if !header.key().is_empty() && !header.value().is_empty() {
                req = req.header(header.key(), header.value());
            }
        }

        if let Some(b) = body {
            req = req.body(b);
        }

        let response =
            tokio::select! {
            res = req.send() => res.map_err(|e| format!("Request failed: {}", e))?,
            _ = token_clone.cancelled() => return Err("Request cancelled".to_string()),
        };

        Ok(HttpResponse::from_response(response).await)
    });

    let result =
        tokio::select! {
        res = handle => res.map_err(|e| e.to_string())?,
        _ = token.cancelled() => Err("Request cancelled".to_string()),
    };

    {
        let mut state = state.lock().await;
        state.cancellation_token = None;
    }

    result
}

pub async fn cancel_request(state: Arc<TokioMutex<AppState>>) -> Result<(), String> {
    let mut app_state = state.lock().await;
    if let Some(token) = &app_state.cancellation_token {
        token.cancel();
        app_state.cancellation_token = None;
        Ok(())
    } else {
        Err("No request to cancel".to_string())
    }
}
