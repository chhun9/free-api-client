use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    parameter_type: String,
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Api {
    id: String,
    name: String,
    method: String,
    url: String,
    headers: Vec<Header>,
    parameters: Vec<Parameter>,
    body: String,
}

impl Header {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl Api {
    pub fn id(&self) -> &str {
        &self.id
    }
}
