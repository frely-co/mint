use axum::{Json, extract::State};

use crate::memory::store::SharedStore;

use super::models::{AdminInitiateAuthRequest, AdminInitiateAuthResponse, SignUpRequest, SignUpResponse};

pub async fn admin_initiate_auth(
    State(store): State<SharedStore>,
    Json(payload): Json<AdminInitiateAuthRequest>,
) -> Json<AdminInitiateAuthResponse> {
    let data = store.write().await;

    // Example: handle authentication logic
    let response = if data.cognito.users.contains_key(&payload.username) {
        AdminInitiateAuthResponse {
            success: true,
            message: "Authentication successful".to_string(),
        }
    } else {
        AdminInitiateAuthResponse {
            success: false,
            message: "User not found".to_string(),
        }
    };

    Json(response)
}

pub async fn signup(
    State(store): State<SharedStore>,
    Json(payload): Json<SignUpRequest>,
) -> Json<SignUpResponse> {
    let mut data = store.write().await;

    // Check if the user already exists
    if data.cognito.users.contains_key(&payload.username) {
        return Json(SignUpResponse {
            success: false,
            message: "User already exists".to_string(),
        });
    }

    // Register the new user
    data.cognito.users.insert(payload.username.clone(), payload.password);

    Json(SignUpResponse {
        success: true,
        message: "User registered successfully".to_string(),
    })
}
