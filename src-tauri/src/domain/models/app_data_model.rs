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

    pub fn find_api(&mut self, id: &str) -> Option<&mut Api> {
        self.apis_mut()
            .iter_mut()
            .find(|api| api.id() == id)
    }

    pub fn find_collection(&mut self, id: &str) -> Option<&mut Collection> {
        self.collections_mut()
            .iter_mut()
            .find(|collection| collection.id() == id)
    }
    
    pub fn find_api_in_collection(&mut self, id: &str) -> Option<&mut Api> {
        self.collections_mut()
            .iter_mut()
            .find_map(|collection|
                collection
                    .apis_mut()
                    .iter_mut()
                    .find(|api| api.id() == id)
            )
    }
}
