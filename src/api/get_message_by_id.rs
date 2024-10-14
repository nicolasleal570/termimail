use crate::errors::GenerateEmailError;

use super::types::full_message_response::MessageResponse;

pub async fn get_message_by_id(
    message_id: String,
    token: &str,
) -> Result<MessageResponse, GenerateEmailError> {
    let url = format!("https://api.mail.tm/messages/{}", message_id);

    let client = reqwest::Client::new();

    let response = client.get(&url).bearer_auth(token).send().await?;

    if response.status().is_success() {
        let message_response: MessageResponse = response.json().await?;
        Ok(message_response)
    } else {
        let error_response_text = response.text().await?;
        Err(GenerateEmailError::ApiError(error_response_text))
    }
}
