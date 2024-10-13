use crate::errors::GenerateEmailError;

use super::types::{account_info::AccountInfo, account_response::AccountResponse};

pub async fn get_account_by_id(
    account_id: String,
    token: String,
) -> Result<AccountInfo, GenerateEmailError> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.mail.tm/accounts/{}", account_id))
        .header("Authorization", format!("Bearer {}", token))
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
