use serde::Serialize;

use crate::errors::GenerateEmailError;

use super::types::{account_info::AccountInfo, account_response::AccountResponse};

/// Represents the request body for creating an account.
#[derive(Serialize, Debug)]
struct AccountRequest {
    address: String,
    password: String,
}

pub async fn create_account(
    email_address: String,
    password: String,
) -> Result<AccountInfo, GenerateEmailError> {
    // Create the request body
    let account_request = AccountRequest {
        address: email_address.clone(),
        password,
    };

    let client = reqwest::Client::new();

    // Send the POST request
    let response = client
        .post("https://api.mail.tm/accounts")
        .json(&account_request)
        .send()
        .await?;

    // Check if the response status is successful
    if response.status().is_success() {
        // Deserialize the successful response
        let account_response: AccountResponse = response.json().await?;

        // Extract the required properties into AccountInfo
        let account_info = AccountInfo {
            id: account_response.id,
            email: account_response.address,
            quota: account_response.quota,
            created_at: account_response.created_at,
            updated_at: account_response.updated_at,
        };

        Ok(account_info)
    } else {
        // Attempt to parse the error message from the response body
        let error_response_text = response.text().await?;
        // Return the error message as an API error
        Err(GenerateEmailError::ApiError(error_response_text))
    }
}
