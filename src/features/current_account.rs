use std::{fs::File, io::Read};

use crate::{
    api::get_account_by_id::get_account_by_id, data_struct::JsonData, errors::GenerateEmailError,
};

pub async fn current_account() -> Result<(), GenerateEmailError> {
    match File::open("data.json") {
        Ok(mut file) => {
            // The file exists; read the email address
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let json_data: JsonData = serde_json::from_str(&contents)?;

            let account_info = get_account_by_id(json_data.id, json_data.token).await?;

            let msg = format!(
                "Email: {} \nQuota: {} \nCreated at: {}",
                account_info.email, account_info.quota, account_info.created_at
            )
            .trim()
            .to_string();

            println!("{}", msg);

            Ok(())
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
