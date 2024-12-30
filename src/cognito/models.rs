use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AdminInitiateAuthRequest {
    /// <p><p>The authentication flow for this call to execute. The API action will depend on this value. For example:</p> <ul> <li> <p> <code>REFRESH<em>TOKEN</em>AUTH</code> will take in a valid refresh token and return new tokens.</p> </li> <li> <p> <code>USER<em>SRP</em>AUTH</code> will take in <code>USERNAME</code> and <code>SRP<em>A</code> and return the SRP variables to be used for next challenge execution.</p> </li> <li> <p> <code>USER</em>PASSWORD<em>AUTH</code> will take in <code>USERNAME</code> and <code>PASSWORD</code> and return the next challenge or tokens.</p> </li> </ul> <p>Valid values include:</p> <ul> <li> <p> <code>USER</em>SRP<em>AUTH</code>: Authentication flow for the Secure Remote Password (SRP) protocol.</p> </li> <li> <p> <code>REFRESH</em>TOKEN<em>AUTH</code>/<code>REFRESH</em>TOKEN</code>: Authentication flow for refreshing the access token and ID token by supplying a valid refresh token.</p> </li> <li> <p> <code>CUSTOM<em>AUTH</code>: Custom authentication flow.</p> </li> <li> <p> <code>ADMIN</em>NO<em>SRP</em>AUTH</code>: Non-SRP authentication flow; you can pass in the USERNAME and PASSWORD directly if the flow is enabled for calling the app client.</p> </li> <li> <p> <code>USER<em>PASSWORD</em>AUTH</code>: Non-SRP authentication flow; USERNAME and PASSWORD are passed directly. If a user migration Lambda trigger is set, this flow will invoke the user migration Lambda if the USERNAME is not found in the user pool. </p> </li> <li> <p> <code>ADMIN<em>USER</em>PASSWORD<em>AUTH</code>: Admin-based user password authentication. This replaces the <code>ADMIN</em>NO<em>SRP</em>AUTH</code> authentication flow. In this flow, Cognito receives the password in the request instead of using the SRP process to verify passwords.</p> </li> </ul></p>
    #[serde(rename = "AuthFlow")]
    pub auth_flow: String,
    /// <p><p>The authentication parameters. These are inputs corresponding to the <code>AuthFlow</code> that you are invoking. The required values depend on the value of <code>AuthFlow</code>:</p> <ul> <li> <p>For <code>USER<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SRP<em>A</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code>.</p> </li> <li> <p>For <code>REFRESH</em>TOKEN<em>AUTH/REFRESH</em>TOKEN</code>: <code>REFRESH<em>TOKEN</code> (required), <code>SECRET</em>HASH</code> (required if the app client is configured with a client secret), <code>DEVICE<em>KEY</code>.</p> </li> <li> <p>For <code>ADMIN</em>NO<em>SRP</em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET<em>HASH</code> (if app client is configured with client secret), <code>PASSWORD</code> (required), <code>DEVICE</em>KEY</code>.</p> </li> <li> <p>For <code>CUSTOM<em>AUTH</code>: <code>USERNAME</code> (required), <code>SECRET</em>HASH</code> (if app client is configured with client secret), <code>DEVICE<em>KEY</code>. To start the authentication flow with password verification, include <code>ChallengeName: SRP</em>A</code> and <code>SRP<em>A: (The SRP</em>A Value)</code>.</p> </li> </ul></p>
    #[serde(rename = "AuthParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The app client ID.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p><p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p> <p>You create custom workflows by assigning AWS Lambda functions to user pool triggers. When you use the AdminInitiateAuth API action, Amazon Cognito invokes the AWS Lambda functions that are specified for various triggers. The ClientMetadata value is passed as input to the functions for only the following triggers:</p> <ul> <li> <p>Pre signup</p> </li> <li> <p>Pre authentication</p> </li> <li> <p>User migration</p> </li> </ul> <p>When Amazon Cognito invokes the functions for these triggers, it passes a JSON payload, which the function receives as input. This payload contains a <code>validationData</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminInitiateAuth request. In your function code in AWS Lambda, you can process the <code>validationData</code> value to enhance your workflow for your specific needs.</p> <p>When you use the AdminInitiateAuth API action, Amazon Cognito also invokes the functions for the following triggers, but it does not provide the ClientMetadata value as input:</p> <ul> <li> <p>Post authentication</p> </li> <li> <p>Custom message</p> </li> <li> <p>Pre token generation</p> </li> <li> <p>Create auth challenge</p> </li> <li> <p>Define auth challenge</p> </li> <li> <p>Verify auth challenge</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html">Customizing User Pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note> <p>Take the following limitations into consideration when you use the ClientMetadata parameter:</p> <ul> <li> <p>Amazon Cognito does not store the ClientMetadata value. This data is available only to AWS Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration does not include triggers, the ClientMetadata parameter serves no purpose.</p> </li> <li> <p>Amazon Cognito does not validate the ClientMetadata value.</p> </li> <li> <p>Amazon Cognito does not encrypt the the ClientMetadata value, so don&#39;t use it to provide sensitive information.</p> </li> </ul> </note></p>
    #[serde(rename = "ClientMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID of the Amazon Cognito user pool.</p>
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,
}

#[derive(Serialize)]
pub struct AdminInitiateAuthResponse {
    #[serde(rename = "AuthenticationResult")]
    pub authentication_result: Option<AuthenticationResultType>,
    pub challenge_name: Option<String>,
    pub challenge_parameters: Option<HashMap<String, String>>,
    pub session: Option<String>,
}

/// <p>Specifies whether the attribute is standard or custom.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AttributeType {
    /// <p>The name of the attribute.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The value of the attribute.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Represents the request to register a user.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SignUpRequest {
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The password of the user you wish to register.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[serde(rename = "SecretHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_hash: Option<String>,
    /// <p>An array of name-value pairs representing user attributes.</p> <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    #[serde(rename = "UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<AttributeType>>,
    /// <p>The user name of the user you wish to register.</p>
    #[serde(rename = "Username")]
    pub username: String,
    /// <p>The validation data in the request to register a user.</p>
    #[serde(rename = "ValidationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<Vec<AttributeType>>,
}

/// <p>The response from the server for a registration request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SignUpResponse {
    /// <p>A response from the server indicating that a user registration has been confirmed.</p>
    #[serde(rename = "UserConfirmed")]
    pub user_confirmed: bool,
    /// <p>The UUID of the authenticated user. This is not the same as <code>username</code>.</p>
    #[serde(rename = "UserSub")]
    pub user_sub: String,
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

