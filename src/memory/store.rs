use tokio::sync::RwLock;
use std::sync::Arc;
use crate::cognito::state::CognitoState;
use std::collections::HashMap;

#[derive(Default)]
pub struct MemoryStore {
    pub cognito: CognitoState,
    pub dynamo_tables: HashMap<String, Vec<String>>,
}

pub type SharedStore = Arc<RwLock<MemoryStore>>;

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            cognito: CognitoState::default(),
            dynamo_tables: HashMap::new(),
        }
    }

    pub fn add_dynamo_table(&mut self, table_name: String) {
        self.dynamo_tables.insert(table_name, Vec::new());
    }

    pub fn get_dynamo_tables(&self) -> &HashMap<String, Vec<String>> {
        &self.dynamo_tables
    }

    pub fn put_item(&mut self, table_name: &str, item: String) -> Option<&Vec<String>> {
        if let Some(table) = self.dynamo_tables.get_mut(table_name) {
            table.push(item);
            Some(table)
        } else {
            None
        }
    }
}
