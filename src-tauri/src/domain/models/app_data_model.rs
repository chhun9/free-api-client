use super::*;
use crate::domain::{ Collection, Api };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppData {
    collections: Vec<Collection>,
    apis: Vec<Api>,
}

impl AppData {
    pub fn new() -> Self {
        AppData {
            collections: Vec::new(),
            apis: Vec::new(),
        }
    }
    pub fn collections_mut(&mut self) -> &mut Vec<Collection> {
        &mut self.collections
    }

    pub fn apis_mut(&mut self) -> &mut Vec<Api> {
        &mut self.apis
    }
}
