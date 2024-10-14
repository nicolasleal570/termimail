use crate::errors::GenerateEmailError;

pub async fn delete_account_by_id(
    account_id: String,
    token: String,
) -> Result<(), GenerateEmailError> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("https://api.mail.tm/accounts/{}", account_id))
        .bearer_auth(token)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_response_text = response.text().await?;
        Err(GenerateEmailError::ApiError(error_response_text))
    }
}
