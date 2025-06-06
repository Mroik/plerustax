use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountField {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub acct: String,
    pub display_name: String,
    pub bot: bool,
    pub note: String,
    pub url: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub fields: Vec<AccountField>,
}
