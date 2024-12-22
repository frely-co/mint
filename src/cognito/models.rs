use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AdminInitiateAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AdminInitiateAuthResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub success: bool,
    pub message: String,
}

