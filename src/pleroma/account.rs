use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountField {
    name: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    id: String,
    acct: String,
    display_name: String,
    bot: bool,
    note: String,
    url: String,
    followers_count: u32,
    following_count: u32,
    statuses_count: u32,
    fields: Vec<AccountField>,
}
