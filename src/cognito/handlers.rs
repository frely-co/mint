use axum::Error;
use uuid::Uuid;
use std::collections::HashMap;
use thiserror::Error;

use super::models::*;
use crate::memory::store::SharedStore;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing AuthParameters")]
    MissingAuthParameters,
    #[error("Missing USERNAME param")]
    MissingUsername,
    #[error("Missing PASSWORD param")]
    MissingPassword,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("User not found")]
    UserNotFound,
}

pub async fn handle_admin_initiate_auth(
    store: &SharedStore,
    payload: AdminInitiateAuthRequest,
) -> Result<AdminInitiateAuthResponse, AuthError> {
    let params = payload.auth_parameters.ok_or(AuthError::MissingAuthParameters)?;
    let username = params.get("USERNAME").ok_or(AuthError::MissingUsername)?;
    let password = params.get("PASSWORD").ok_or(AuthError::MissingPassword)?;

    let mut data = store.write().await;
    let user_map = &mut data.cognito.users;

    match user_map.get(username) {
        Some(stored_pass) if stored_pass == password => {
            let authentication_result = AuthenticationResultType {
                access_token: "access_token".to_owned(),
                expires_in: 3600,
                token_type: "Bearer".to_owned(),
                refresh_token: "refresh_token".to_owned(),
                id_token: "id_token".to_owned(),
                new_device_metadata: NewDeviceMetadataType {
                    device_key: "device_key".to_owned(),
                    device_group_key: "device_group_key".to_owned(),
                },
            };
            Ok(AdminInitiateAuthResponse {
                authentication_result: Some(authentication_result),
                challenge_name: Some("challenge_name".to_string()),
                challenge_parameters: Some(HashMap::new()),
                session: Some("session".to_string()),
            })
        }
        Some(_) => Err(AuthError::InvalidPassword),
        None => Err(AuthError::UserNotFound),
    }
}

pub async fn handle_signup(store: &SharedStore, payload: SignUpRequest) -> Result<SignUpResponse, Error> {
    let mut data = store.write().await;
    let user_map = &mut data.cognito.users;

    let user_confirmed = !user_map.contains_key(&payload.username);
    if user_confirmed {
        user_map.insert(payload.username.clone(), payload.password);
    }

    Ok(SignUpResponse {
        user_confirmed,
        user_sub: Uuid::new_v4().to_string(),
    })
}
