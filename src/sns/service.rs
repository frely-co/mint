use crate::memory::store::SharedStore;
use crate::sns::models::*;

#[derive(Clone)]
pub struct SnsService {
    store: SharedStore,
}

impl SnsService {
    pub fn new(store: SharedStore) -> Self {
        Self { store }
    }

    pub async fn create_topic(&self, name: String) -> String {
        let mut data = self.store.write().await;
        let topic_arn = format!("arn:aws:sns:local:000000000000:{}", name);
        data.sns.topics.insert(
            topic_arn.clone(),
            Topic {
                topic_arn: topic_arn.clone(),
                name: name.clone(),
            },
        );
        topic_arn
    }

    pub async fn publish(&self, topic_arn: String) -> bool {
        let data = self.store.read().await;
        data.sns.topics.contains_key(&topic_arn)
    }

    pub async fn list_topics(&self) -> Vec<Topic> {
        let data = self.store.read().await;
        data.sns.topics.values().cloned().collect()
    }

    pub async fn delete_topic(&self, topic_arn: String) -> bool {
        let mut data = self.store.write().await;
        data.sns.topics.remove(&topic_arn).is_some()
    }
}
