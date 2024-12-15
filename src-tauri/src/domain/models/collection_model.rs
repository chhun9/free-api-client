use super::*;
use crate::domain::Api;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    id: String,
    name: String,
    apis: Vec<Api>,
}

impl Collection {
    pub fn apis_mut(&mut self) -> &mut Vec<Api> {
        &mut self.apis
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}
