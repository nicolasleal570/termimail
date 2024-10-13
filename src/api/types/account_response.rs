use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountResponse {
    #[serde(rename = "@context")]
    pub _context: String,
    #[serde(rename = "@id")]
    pub _id_url: String,
    #[serde(rename = "@type")]
    pub _type_field: String,
    pub id: String,
    pub address: String,
    pub quota: u64,
    #[serde(rename = "used")]
    pub _used: u64,
    #[serde(rename = "isDisabled")]
    pub _is_disabled: bool,
    #[serde(rename = "isDeleted")]
    pub _is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
