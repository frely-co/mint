use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::{from_slice, json};

use crate::memory::store::SharedStore;

use super::{
    handlers::{handle_admin_initiate_auth, handle_signup},
    models::{AdminInitiateAuthRequest, SignUpRequest, SnsPublishRequest},
};

pub async fn dispatch_cognito(
    x_amz_target: &str,
    store: SharedStore,
    body_bytes: impl AsRef<[u8]>,
) -> Response {
    eprintln!("x_amz_target: {}", x_amz_target);
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

        "SNS.Publish" => {
            println!("testeee");
            let payload: SnsPublishRequest = match serde_json::from_slice(body_bytes.as_ref()) {
                Ok(p) => {
                    eprintln!("Successfully parsed SnsPublishRequest: {:?}", p);
                    p
                }
                Err(e) => {
                    eprintln!("Failed to parse SnsPublishRequest: {}", e);
                    return (
                        StatusCode::BAD_REQUEST,
                        "Invalid JSON for SnsPublishRequest",
                    )
                        .into_response();
                }
            };

            // Here you would handle the SNS publish logic, e.g., store the message in the shared store
            // and trigger any subscribed Lambdas or other services.
            // For simplicity, we'll just return a success response.
            let response = json!({
                "MessageId": "example-message-id"
            });

            eprintln!("SNS publish response: {:?}", response);
            Json(response).into_response()
        }

        // If we don’t recognize the operation
        _ => {
            let msg = format!("Unknown Cognito X-Amz-Target: {x_amz_target}");
            (StatusCode::BAD_REQUEST, msg).into_response()
        }
    }
}
