use std::collections::HashMap;

#[derive(Default)]
pub struct CognitoState {
    pub users: HashMap<String, String>, // username -> password
}
