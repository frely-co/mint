use core::panic;

use aws_sdk_cognitoidentityprovider::types::builders::AuthenticationResultTypeBuilder;
use axum::body::Bytes;
use axum::http::HeaderMap;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde_json::Value;

use crate::memory::store::SharedStore;

use super::models::{AuthenticationResultType, NewDeviceMetadataType};
use super::models::{SignUpRequest, SignUpResponse, AdminInitiateAuthRequest};

#[debug_handler]
pub async fn admin_initiate_auth(
    State(store): State<SharedStore>,
    headers: HeaderMap,
    body_bytes: Bytes  // use this and refactor the code
    // Json(payload): Json(AdminInitiateAuthRequest),
) -> Json<AuthenticationResultType> {
    // Try to get the "X-Amz-Target" header
    println!("Primeiro passo aqui");
    let x_amz_target = headers
        .get("X-Amz-Target")
        .and_then(|val| val.to_str().ok()) // convert to &str if valid UTF-8
        .unwrap_or("");

    // Check that the X-Amz-Target header matches what you expect
    if x_amz_target != "AWSCognitoIdentityProviderService.AdminInitiateAuth" {
        panic!("panic")
    }

    // Extract AuthParameters from the JSON payload
    let auth_parameters = payload.AuthParameters;
    if let Some(params) = auth_parameters {
        let username = params
            .get("USERNAME").unwrap();
        let password = params
            .get("PASSWORD").unwrap();

        let data = store.write().await;

        // Validate user credentials
        let response = if let Some(stored_password) = data.cognito.users.get(username) {
            if stored_password == password {
                println!("Entoru aqui");
                let response = AuthenticationResultType {
                    access_token: "access_token".to_string(),
                    expires_in: 123,
                    token_type: "token_type".to_string(),
                    refresh_token: "refresh_token".to_string(),
                    id_token: "id_token".to_string(),
                    new_device_metadata: NewDeviceMetadataType {
                        device_key: "device_key".to_string(),
                        device_group_key: "device_key_group".to_string(),
                    },
                };
                return Json(response);
            } else {
                panic!("panic")
            }
        } else {
            panic!("panic")
        };
    } else {
        panic!("panic")
    }
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
    data.cognito
        .users
        .insert(payload.username.clone(), payload.password);

    Json(SignUpResponse {
        success: true,
        message: "User registered successfully".to_string(),
    })
}
