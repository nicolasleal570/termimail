use serde::{Deserialize, Serialize};

use crate::errors::GenerateEmailError;

/// Represents the request body for creating a token.
#[derive(Serialize, Debug)]
struct TokenRequest {
    address: String,
    password: String,
}

/// Represents the response from the API after creating a token.
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
    #[serde(rename = "@id")]
    pub _id_url: String,
    #[serde(rename = "id")]
    pub _id: String,
}

pub async fn create_token(
    email_address: String,
    password: String,
) -> Result<TokenResponse, GenerateEmailError> {
    // Create the request body
    let token_request = TokenRequest {
        address: email_address.clone(),
        password,
    };

    let client = reqwest::Client::new();

    // Send the POST request
    let response = client
        .post("https://api.mail.tm/token")
        .json(&token_request)
        .send()
        .await?;

    // Check if the response status is successful
    if response.status().is_success() {
        // Deserialize the successful response
        let token_response: TokenResponse = response.json().await?;
        Ok(token_response)
    } else {
        // Attempt to parse the error message from the response body
        let error_response_text = response.text().await?;
        // Return the error message as an API error
        Err(GenerateEmailError::ApiError(error_response_text))
    }
}
