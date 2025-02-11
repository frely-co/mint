use crate::dynamodb::models::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        self.tables.insert(
            table_name.clone(),
            DynamoTable {
                table_name: table_name.clone(),
                items: HashMap::new(),
            },
        );
    }

    pub fn put_item(
        &mut self,
        table_name: &str,
        item: HashMap<String, AttributeValue>,
    ) -> Option<&HashMap<String, AttributeValue>> {
        if let Some(table) = self.tables.get_mut(table_name) {
            let key = item.get("id").and_then(|v| v.s.clone()).unwrap_or_default();
            table.items.insert(key.clone(), item);
            table.items.get(&key)
        } else {
            None
        }
    }

    pub fn get_item(
        &self,
        table_name: &str,
        key: &str,
    ) -> Option<&HashMap<String, AttributeValue>> {
        self.tables
            .get(table_name)
            .and_then(|table| table.items.get(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamodb::models::AttributeValue;

    #[test]
    fn test_create_table() {
        let mut repo = DynamoRepository::new();
        repo.create_table("test_table".to_string());

        assert!(repo.tables.contains_key("test_table"));
        assert_eq!(repo.tables["test_table"].table_name, "test_table");
        assert!(repo.tables["test_table"].items.is_empty());
    }

    #[test]
    fn test_put_item() {
        let mut repo = DynamoRepository::new();
        repo.create_table("test_table".to_string());

        let mut item = HashMap::new();
        item.insert(
            "id".to_string(),
            AttributeValue {
                s: Some("123".to_string()),
                ..Default::default()
            },
        );
        item.insert(
            "name".to_string(),
            AttributeValue {
                s: Some("test".to_string()),
                ..Default::default()
            },
        );

        let result = repo.put_item("test_table", item.clone());

        assert!(result.is_some());
        assert_eq!(repo.tables["test_table"].items.get("123"), Some(&item));
    }

    #[test]
    fn test_put_item_nonexistent_table() {
        let mut repo = DynamoRepository::new();

        let mut item = HashMap::new();
        item.insert(
            "id".to_string(),
            AttributeValue {
                s: Some("123".to_string()),
                ..Default::default()
            },
        );

        let result = repo.put_item("nonexistent_table", item);

        assert!(result.is_none());
    }

    #[test]
    fn test_get_item() {
        let mut repo = DynamoRepository::new();
        repo.create_table("test_table".to_string());

        let mut item = HashMap::new();
        item.insert(
            "id".to_string(),
            AttributeValue {
                s: Some("123".to_string()),
                ..Default::default()
            },
        );
        item.insert(
            "name".to_string(),
            AttributeValue {
                s: Some("test".to_string()),
                ..Default::default()
            },
        );

        repo.put_item("test_table", item.clone());

        let retrieved_item = repo.get_item("test_table", "123");

        assert!(retrieved_item.is_some());
        assert_eq!(retrieved_item, Some(&item));
    }

    #[test]
    fn test_get_item_nonexistent_table() {
        let repo = DynamoRepository::new();

        let retrieved_item = repo.get_item("nonexistent_table", "123");

        assert!(retrieved_item.is_none());
    }

    #[test]
    fn test_get_item_nonexistent_key() {
        let mut repo = DynamoRepository::new();
        repo.create_table("test_table".to_string());

        let retrieved_item = repo.get_item("test_table", "nonexistent_key");

        assert!(retrieved_item.is_none());
    }
}
