use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower::ServiceBuilder;

#[derive(Debug, Deserialize)]
struct AdminInitiateAuthRequest {
    UserPoolId: String,
    ClientId: String,
    AuthFlow: String,
    AuthParameters: Option<std::collections::HashMap<String, String>>,
}

// A helper function that handles the AdminInitiateAuth business logic
async fn handle_admin_initiate_auth(req: AdminInitiateAuthRequest) -> Response {
    // Here you can add logic: check if username/password matches expected values,
    // return errors if needed, etc.

    // For now, return a hard-coded authentication result.
    let response = json!({
        "AuthenticationResult": {
            "AccessToken": "mockedAccessToken",
            "ExpiresIn": 3600,
            "TokenType": "Bearer",
            "RefreshToken": "mockedRefreshToken",
            "IdToken": "mockedIdToken"
        },
        "ChallengeParameters": {}
    });

    (StatusCode::OK, response.to_string()).into_response()
}

// This is our main handler that will match on the X-Amz-Target header and parse the body.
async fn cognito_handler(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
    // Check the X-Amz-Target header to determine which action we should simulate
    let target = headers
        .get("X-Amz-Target")
        .and_then(|val| val.to_str().ok())
        .unwrap_or_default();

    // For Cognito: AdminInitiateAuth call
    if target == "AWSCognitoIdentityProviderService.AdminInitiateAuth" {
        // Parse JSON
        let parsed: Result<AdminInitiateAuthRequest, _> = serde_json::from_slice(&body);
        match parsed {
            Ok(request) => handle_admin_initiate_auth(request).await,
            Err(e) => {
                eprintln!("Failed to parse request: {}", e);
                (StatusCode::BAD_REQUEST, "Invalid request body").into_response()
            }
        }
    } else {
        // Unknown target => Return an error or handle other calls
        (StatusCode::BAD_REQUEST, "Unknown X-Amz-Target").into_response()
    }
}

#[tokio::main]
async fn main() {
    // Construct the app: a single POST endpoint that simulates the AWS Cognito endpoint.
    let app = Router::new()
        .route("/", post(cognito_handler))
        .layer(ServiceBuilder::new());

    // Run the server on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Mock AWS Cognito service running at http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

