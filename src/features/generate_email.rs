use std::fs::File;
use std::io::{Read, Write};

use rand::distributions::{Alphanumeric, DistString};

use crate::api::create_account::create_account;
use crate::api::create_token::create_token;
use crate::api::get_domains::get_domains;
use crate::data_struct::JsonData;
use crate::errors::GenerateEmailError;

pub async fn generate_email() -> Result<(), GenerateEmailError> {
    let domains_list = get_domains().await?;

    if let Some(domain_item) = domains_list.get(0) {
        // Proceed with domain_item
        match File::open("data.json") {
            Ok(mut file) => {
                // The file exists; read the email address
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let json_data: JsonData = serde_json::from_str(&contents)?;
                return Err(GenerateEmailError::ExistingAccount(json_data.email));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // File does not exist; create an empty JSON
                let domain = domain_item.domain.clone();

                let email_username = Alphanumeric
                    .sample_string(&mut rand::thread_rng(), 7)
                    .to_lowercase();

                let email = format!("{email_username}@{domain}");
                let password = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);

                // Create account
                let account_info = create_account(email.clone(), password.clone()).await?;

                // Create token
                let token_response = create_token(email.clone(), password.clone()).await?;

                // Create body data
                let user_data = JsonData {
                    id: account_info.id,
                    email: account_info.email,
                    password,
                    token: token_response.token,
                    is_disabled: false,
                    is_deleted: false,
                    quota: account_info.quota.to_string(),
                    created_at: account_info.created_at,
                    updated_at: account_info.updated_at,
                };

                // Serialize the data to a JSON string
                let json_string = serde_json::to_string_pretty(&user_data)?;

                // Write the JSON string to "data.json"
                let mut file = File::create("data.json")?;
                file.write_all(json_string.as_bytes())?;
            }
            Err(e) => {
                eprintln!("Failed to open data.json: {:?}", e);
                return Err(e.into());
            }
        }
    } else {
        println!("There's no valid domain available");
    }

    Ok(())
}
