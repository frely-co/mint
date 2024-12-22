use tokio::sync::RwLock;
use std::sync::Arc;
use crate::cognito::state::CognitoState;

#[derive(Default)]
pub struct MemoryStore {
    pub cognito: CognitoState,
}

pub type SharedStore = Arc<RwLock<MemoryStore>>;

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            cognito: CognitoState::default(),
        }
    }
}

