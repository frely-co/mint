use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AdminInitiateAuthRequest {
    pub UserPoolId: String,
    pub ClientId: String,
    pub AuthFlow: String,
    pub AuthParameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize)]
pub struct AdminInitiateAuthResponse {
    #[serde(rename = "AuthenticationResult")]
    pub authentication_result: Option<AuthenticationResultType>,
    pub challenge_name: Option<String>,
    pub challenge_parameters: Option<HashMap<String, String>>,
    pub session: Option<String>,
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

#[derive(Serialize)]
pub struct AuthenticationResultType {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
    pub refresh_token: String,
    #[serde(rename = "IdToken")]
    pub id_token: String,
    pub new_device_metadata: NewDeviceMetadataType,
}

#[derive(Serialize)]
pub struct NewDeviceMetadataType {
    pub device_key: String,
    pub device_group_key: String,
}
