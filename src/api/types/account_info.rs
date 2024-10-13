/// Contains the desired properties extracted from the account response.
#[derive(Debug)]
pub struct AccountInfo {
    pub id: String,
    pub email: String,
    pub quota: u64,
    pub created_at: String,
    pub updated_at: String,
}
