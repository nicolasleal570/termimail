use std::{fs::File, io::Read};

use inquire::{InquireError, Select};

use crate::{
    api::get_messages::get_messages, data_struct::JsonData, errors::GenerateEmailError,
    utils::open_email::open_email,
};

pub async fn fetch_emails() -> Result<(), GenerateEmailError> {
    match File::open("data.json") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let json_data: JsonData = serde_json::from_str(&contents)?;

            let messages_payload = get_messages(&json_data.token).await?;

            if messages_payload.messages.len() == 0 {
                println!("No Emails.");
                return Ok(());
            }

            let ans = Select::new("Your inbox:", messages_payload.messages).prompt();

            match ans {
                Ok(selected_email) => {
                    open_email(selected_email, &json_data.token).await?;

                    Ok(())
                }
                Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                    // Handle Ctrl+C or cancellation gracefully
                    //println!("Operation interrupted by user.");
                    return Ok(());
                }
                Err(err) => {
                    // Handle other errors
                    eprintln!("Failed to select your email: {}", err);
                    // Return an error or handle it accordingly
                    return Err(GenerateEmailError::ApiError(err.to_string()));
                }
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(GenerateEmailError::ApiError(
                "Account not created yet".to_string(),
            ));
        }
        Err(e) => {
            eprintln!("Failed to open data.json: {:?}", e);
            return Err(e.into());
        }
    }
}
