use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountField {
    name: String,
    value: String,
}

#[derive(Deserialize)]
pub struct Account {
    id: String,
    acct: String,
    display_name: String,
    bot: bool,
    note: String,
    url: String,
    follower_count: u32,
    following_count: u32,
    statuses_count: u32,
    fields: Vec<AccountField>,
}
