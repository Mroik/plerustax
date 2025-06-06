use anyhow::anyhow;
use serde::Deserialize;

use super::account::Account;

#[derive(Deserialize, Debug)]
pub struct MediaAttatchmentRaw {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    url: String,
    preview_url: String,
    description: Option<String>,
}

pub enum MediaAttatchment {
    Image {
        id: String,
        url: String,
        preview_url: String,
        description: Option<String>,
    },
    Video {
        id: String,
        url: String,
        preview_url: String,
        description: Option<String>,
    },
    Gifv {
        id: String,
        url: String,
        preview_url: String,
        description: Option<String>,
    },
    Audio {
        id: String,
        url: String,
        preview_url: String,
        description: Option<String>,
    },
}

impl TryFrom<MediaAttatchmentRaw> for MediaAttatchment {
    type Error = anyhow::Error;

    fn try_from(value: MediaAttatchmentRaw) -> Result<Self, Self::Error> {
        match value.type_.as_str() {
            "image" => Ok(MediaAttatchment::Image {
                id: value.id,
                url: value.url,
                preview_url: value.preview_url,
                description: value.description,
            }),
            "video" => Ok(MediaAttatchment::Video {
                id: value.id,
                url: value.url,
                preview_url: value.preview_url,
                description: value.description,
            }),
            "gifv" => Ok(MediaAttatchment::Gifv {
                id: value.id,
                url: value.url,
                preview_url: value.preview_url,
                description: value.description,
            }),
            "audio" => Ok(MediaAttatchment::Audio {
                id: value.id,
                url: value.url,
                preview_url: value.preview_url,
                description: value.description,
            }),
            _ => Err(anyhow!("Invalid type: {}", value.type_)),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TweetMention {
    id: String,
    acct: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct TweetTag {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct PollOption {
    title: String,
    votes_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Poll {
    id: String,
    expires_at: String,
    expired: bool,
    multiple: bool,
    votes_count: u32,
    voters_count: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Tweet {
    pub id: String,
    pub created_at: String,
    pub in_response_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    pub uri: String,
    pub replies_count: u32,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub favourited: bool,
    pub reblogged: bool,
    pub muted: bool,
    pub content: String,
    pub reblog: Option<Box<Tweet>>,
    pub account: Account,
    pub media_attachments: Vec<MediaAttatchmentRaw>,
    pub mentions: Vec<TweetMention>,
    pub tags: Vec<TweetTag>,
    pub poll: Option<Poll>,
}

impl Tweet {
    // TODO
}
