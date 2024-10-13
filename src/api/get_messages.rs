use crate::errors::GenerateEmailError;

use super::types::messages_response::MessagesResponse;

pub async fn get_messages(token: String) -> Result<MessagesResponse, GenerateEmailError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.mail.tm/messages")
        .bearer_auth(token)
        .send()
        .await?;

    // Check if the response status is successful
    if response.status().is_success() {
        // Deserialize the successful response
        let messages: MessagesResponse = response.json().await?;

        Ok(messages)
    } else {
        // Attempt to parse the error message from the response body
        let error_response_text = response.text().await?;
        // Return the error message as an API error
        Err(GenerateEmailError::ApiError(error_response_text))
    }
}
