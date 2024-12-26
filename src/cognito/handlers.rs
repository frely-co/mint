use uuid::Uuid;

use super::models::*;
use crate::memory::store::SharedStore;
use std::{collections::HashMap, error::Error};

pub async fn handle_admin_initiate_auth(
    store: &SharedStore,
    payload: AdminInitiateAuthRequest,
) -> Result<AdminInitiateAuthResponse, Box<dyn Error>> {
    let params = match payload.auth_parameters {
        Some(p) => p,
        None => return Err("Missing AuthParameters".into()),
    };

    let username = params.get("USERNAME").ok_or("Missing USERNAME param")?;
    let password = params.get("PASSWORD").ok_or("Missing PASSWORD param")?;

    let mut data = store.write().await;
    let user_map = &mut data.cognito.users;

    if let Some(stored_pass) = user_map.get(username) {
        if stored_pass == password {
            // Return a success result
            let authentication_result_type = AuthenticationResultType {
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
                authentication_result: Some(authentication_result_type),
                challenge_name: Some("challenge_name".to_string()),
                challenge_parameters: Some(HashMap::new()),
                session: Some("session".to_string()),
            })
        } else {
            Err("Invalid password".into())
        }
    } else {
        Err("User not found".into())
    }
}

pub async fn handle_signup(store: &SharedStore, payload: SignUpRequest) -> SignUpResponse {
    let mut data = store.write().await;
    let user_map = &mut data.cognito.users;

    if user_map.contains_key(&payload.username) {
        return SignUpResponse {
            user_confirmed: false,
            user_sub: Uuid::new_v4().to_string(),
        };
    }

    user_map.insert(payload.username.clone(), payload.password);

    return SignUpResponse {
        user_confirmed: true,
        user_sub: Uuid::new_v4().to_string(),
    };
}
