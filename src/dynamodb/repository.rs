use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dynamodb::models::AttributeValue;

#[derive(Default)]
pub struct DynamoTable {
    pub table_name: String,
    pub items: HashMap<String, HashMap<String, AttributeValue>>,
}

#[derive(Default)]
pub struct DynamoRepository {
    pub tables: HashMap<String, DynamoTable>,
}

impl DynamoRepository {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, table_name: String) {
        self.tables.insert(table_name.clone(), DynamoTable {
            table_name: table_name.clone(),
            items: HashMap::new(),
        });
    }

    pub fn put_item(
        &mut self,
        table_name: &str,
        item: HashMap<String, AttributeValue>,
    ) -> Option<&HashMap<String, AttributeValue>> {
        if let Some(table) = self.tables.get_mut(table_name) {
            let key = item.get("id").and_then(|v| v.s.clone()).unwrap_or_default();
            table.items.insert(key, item);
            Some(&table.items)
        } else {
            None
        }
    }

    pub fn get_item(
        &self,
        table_name: &str,
        key: &str,
    ) -> Option<&HashMap<String, AttributeValue>> {
        self.tables.get(table_name).and_then(|table| table.items.get(key))
    }
}

