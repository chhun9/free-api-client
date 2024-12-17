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
    pub fn new_id(&mut self) -> String {
        self.id = Uuid::new_v4().to_string();
        self.id.clone()
    }
    pub fn new_name(&mut self, name: String) {
        self.name = name;
    }
}
