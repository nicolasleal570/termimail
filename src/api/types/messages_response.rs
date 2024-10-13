use serde::Deserialize;

/// Represents the response from the API when retrieving messages.
#[derive(Deserialize, Debug)]
pub struct MessagesResponse {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id_url: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    #[serde(rename = "hydra:totalItems")]
    pub total_items: u64,
    #[serde(rename = "hydra:member")]
    pub messages: Vec<Message>,
}

/// Represents an individual message.
#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "@id")]
    pub id_url: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub id: String,
    pub msgid: String,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub subject: String,
    pub intro: String,
    pub seen: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    pub size: u64,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from_name: String = if !self.from.name.is_empty() {
            self.from.name.clone()
        } else {
            self.from.address.clone()
        };
        write!(f, "{} - From: {}", self.subject, from_name)
    }
}
/// Represents an email address with an optional name.
#[derive(Deserialize, Debug)]
pub struct EmailAddress {
    pub address: String,
    pub name: String,
}
