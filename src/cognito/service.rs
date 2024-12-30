use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::from_slice;

use crate::memory::store::SharedStore;

use super::{
    handlers::{handle_admin_initiate_auth, handle_signup},
    models::{AdminInitiateAuthRequest, SignUpRequest},
};

pub async fn dispatch_cognito(
    x_amz_target: &str,
    store: SharedStore,
    body_bytes: impl AsRef<[u8]>,
) -> Response {
    match x_amz_target {
        "AWSCognitoIdentityProviderService.AdminInitiateAuth" => {
            // Parse the body
            let payload: AdminInitiateAuthRequest = match from_slice(body_bytes.as_ref()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse AdminInitiateAuthRequest: {e}");
                    return (
                        StatusCode::BAD_REQUEST,
                        "Invalid JSON for AdminInitiateAuthRequest",
                    )
                        .into_response();
                }
            };

            match handle_admin_initiate_auth(&store, payload).await {
                Ok(auth_result) => Json(auth_result).into_response(),
                Err(err) => {
                    eprintln!("Error in AdminInitiateAuth: {}", err);
                    (StatusCode::BAD_REQUEST, err.to_string()).into_response()
                }
            }
        }

        "AWSCognitoIdentityProviderService.SignUp" => {
            let payload: SignUpRequest = match from_slice(body_bytes.as_ref()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to parse SignUpRequest: {e}");
                    return (StatusCode::BAD_REQUEST, "Invalid JSON for SignUpRequest")
                        .into_response();
                }
            };

            let sign_up_response = handle_signup(&store, payload).await;

            match sign_up_response {
                Ok(response) => {
                    eprintln!("SignUp successful");
                    Json(response).into_response()
                }
                Err(e) => {
                    eprintln!("SignUp failed: {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "SignUp failed").into_response()
                }
            }
        }

        // If we don’t recognize the operation
        _ => {
            let msg = format!("Unknown Cognito X-Amz-Target: {x_amz_target}");
            (StatusCode::BAD_REQUEST, msg).into_response()
        }
    }
}
