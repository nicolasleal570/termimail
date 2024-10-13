use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData {
    pub id: String,
    pub email: String,
    pub password: String,
    pub quota: String,
    pub token: String,
    #[serde(rename = "isDisabled")]
    pub is_disabled: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
