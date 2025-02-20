use tokio::sync::RwLock;
use std::sync::Arc;
use crate::cognito::state::CognitoState;
use crate::dynamodb::repository::DynamoRepository;
use crate::sns::repository::SnsRepository;

#[derive(Default)]
pub struct MemoryStore {
    pub cognito: CognitoState,
    pub dynamo: DynamoRepository,
    pub sns: SnsRepository,
}

pub type SharedStore = Arc<RwLock<MemoryStore>>;

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            cognito: CognitoState::default(),
            dynamo: DynamoRepository::new(),
            sns: SnsRepository::new(),
        }
    }
}
