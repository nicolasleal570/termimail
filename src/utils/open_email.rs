use std::{fs::File, io::Write};

use crate::{
    api::{get_message_by_id::get_message_by_id, types::messages_response::Message},
    errors::GenerateEmailError,
};

pub async fn open_email(
    selected_email: Message,
    account_token: &str,
) -> Result<(), GenerateEmailError> {
    let message_payload = get_message_by_id(selected_email.id, &account_token).await?;

    let html_vec = message_payload.html.expect("No HTML content found");

    let html_content = html_vec.get(0).expect("No HTML content found");

    let filename = "data.html";
    let mut temp_path = std::env::current_dir()?;
    temp_path.push(filename);

    let mut temp_file = File::create(&temp_path)?;
    temp_file.write_all(html_content.as_bytes())?;

    webbrowser::open(&temp_path.to_string_lossy())?;

    Ok(())
}
