use serde_json::Value;
use reqwest::header::HeaderMap;
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HeaderMap,
    pub body: Option<Value>,
}