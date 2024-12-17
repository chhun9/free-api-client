use super::*;
use serde_json::Value;
use reqwest::header::{ HeaderMap, HeaderValue };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpResponse {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Option<Value>,
}

impl HttpResponse {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status_code = response.status().as_u16();

        let headers = response
            .headers()
            .iter()
            .filter_map(|(key, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|v| (key.to_string(), v.to_string()))
            })
            .collect();

        let body = response.json::<serde_json::Value>().await.ok();

        HttpResponse {
            status_code,
            headers,
            body,
        }
    }
}
