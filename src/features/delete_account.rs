use std::{
    fs::{self, File},
    io::Read,
};

use comfy_table::Table;

use crate::{
    api::delete_account_by_id::delete_account_by_id, data_struct::JsonData,
    errors::GenerateEmailError,
};

pub async fn delete_account() -> Result<(), GenerateEmailError> {
    match File::open("data.json") {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let json_data: JsonData = serde_json::from_str(&contents)?;

            delete_account_by_id(json_data.id, json_data.token).await?;

            println!("Deleted account.");

            let _ = fs::remove_file("data.json");

            let mut table = Table::new();

            table
                .set_header(vec!["Email"])
                .add_row(vec![json_data.email]);

            println!("{table}");

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
