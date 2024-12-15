use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum HttpParameter {
    PATH,
    QUERY,
}

impl HttpParameter {
    pub fn as_str(&self) -> &str {
        match self {
            HttpParameter::PATH => "PATH",
            HttpParameter::QUERY => "QUERY",
        }
    }
    pub fn from_str(parameter: &str) -> HttpParameter {
        for parameter_value in HttpParameter::iter() {
            if parameter_value.as_str() == parameter {
                return parameter_value;
            }
        }
        HttpParameter::PATH
    }
    pub fn get_all_parameters() -> Vec<String> {
        HttpParameter::iter()
            .map(|m| m.as_str().to_string())
            .collect()
    }
}
