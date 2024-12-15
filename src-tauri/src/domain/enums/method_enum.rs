use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
        }
    }
    pub fn from_str(method: &str) -> HttpMethod {
        for method_enum in HttpMethod::iter() {
            if method_enum.as_str() == method {
                return method_enum;
            }
        }
        HttpMethod::GET
    }
    pub fn get_all_methods() -> Vec<String> {
        HttpMethod::iter()
            .map(|m| m.as_str().to_string())
            .collect()
    }
}
