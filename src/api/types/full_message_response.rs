use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageResponse {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id_url: String,
    #[serde(rename = "@type")]
    pub type_field: String,
    pub id: String,
    pub msgid: String,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: String,
    pub intro: String,
    pub seen: bool,
    pub flagged: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    pub verifications: Verifications,
    pub retention: bool,
    #[serde(rename = "retentionDate")]
    pub retention_date: Option<String>,
    pub text: Option<String>,
    pub html: Option<Vec<String>>,
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

/// Represents an email address with an optional name.
#[derive(Deserialize, Debug)]
pub struct EmailAddress {
    pub address: String,
    pub name: String,
}

/// Represents verifications for the message.
#[derive(Deserialize, Debug)]
pub struct Verifications {
    pub tls: TlsVerification,
    pub spf: bool,
    pub dkim: bool,
}

/// Represents TLS verification details.
#[derive(Deserialize, Debug)]
pub struct TlsVerification {
    pub name: String,
    #[serde(rename = "standardName")]
    pub standard_name: String,
    pub version: String,
}
