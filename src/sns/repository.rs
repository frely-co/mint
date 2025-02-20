use std::collections::HashMap;
use crate::sns::models::Topic;

#[derive(Default)]
pub struct SnsRepository {
    pub topics: HashMap<String, Topic>,
}

impl SnsRepository {
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }
}
