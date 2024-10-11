use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "hydra:totalItems")]
    pub total_items: u32,
    #[serde(rename = "hydra:member")]
    pub domains: Vec<Domain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    #[serde(rename = "@id")]
    pub at_id: String,
    #[serde(rename = "@type")]
    pub at_type: String,
    pub id: String,
    pub domain: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub async fn get_domains() -> Result<Vec<Domain>, reqwest::Error> {
    let response = reqwest::get("https://api.mail.tm/domains")
        .await?
        .json::<APIResponse>()
        .await?;
    Ok(response.domains)
}
