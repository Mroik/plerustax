use serde::Deserialize;

use super::account::Account;

// TODO
#[derive(Deserialize)]
pub struct MediaAttatchment {}

#[derive(Deserialize)]
pub struct TweetMention {
    id: String,
    acct: String,
    url: String,
}

#[derive(Deserialize)]
pub struct TweetTag {
    name: String,
}

#[derive(Deserialize)]
pub struct PollOption {
    title: String,
    votes_count: u32,
}

#[derive(Deserialize)]
pub struct Poll {
    id: String,
    expires_at: String,
    expired: bool,
    multiple: bool,
    votes_count: u32,
    voters_count: Option<u32>,
}

#[derive(Deserialize)]
pub struct Tweet {
    id: String,
    created_at: String,
    in_response_to_id: Option<String>,
    in_reply_to_account_id: Option<String>,
    sensitive: bool,
    spoiler_text: String,
    visibility: String,
    uri: String,
    replies_count: u32,
    reblog_count: u32,
    favourites_count: u32,
    favourited: bool,
    reblogged: bool,
    muted: bool,
    content: String,
    reblog: Option<Box<Tweet>>,
    account: Account,
    media_attachments: Vec<MediaAttatchment>,
    mentions: Vec<TweetMention>,
    tags: Vec<TweetTag>,
    poll: Option<Poll>,
}
