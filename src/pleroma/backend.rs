use std::collections::HashMap;

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CredentialApplication {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

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

#[derive(Debug)]
pub struct Backend {
    base_url: String,
    http: Client,
    credentials: Option<CredentialApplication>,
    token: Option<String>,
}

impl Backend {
    pub async fn new(base_url: &str) -> Result<Self> {
        let mut ris = Backend {
            base_url: base_url.to_string(),
            http: Client::new(),
            credentials: None,
            token: None,
        };

        let mut data = HashMap::new();
        data.insert("client_name", "plerustax");
        data.insert("scopes", "read write");
        data.insert("redirect_uris", "urn:ietf:wg:oauth:2.0:oob");

        let res = ris
            .http
            .post(format!("{}/api/v1/apps", ris.base_url.as_str()))
            .json(&data)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "Status code response: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await.unwrap()
            ));
        }

        let data: CredentialApplication = res.json().await?;
        ris.credentials = Some(data);

        Ok(ris)
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        let mut data = HashMap::new();
        data.insert(
            "client_id",
            self.credentials.as_ref().unwrap().client_id.as_str(),
        );
        data.insert(
            "client_secret",
            self.credentials.as_ref().unwrap().client_secret.as_str(),
        );
        data.insert("redirect_uri", "urn:ietf:wg:oauth:2.0:oob");
        data.insert("grant_type", "password");
        data.insert("username", username);
        data.insert("password", password);

        let res = self
            .http
            .post(format!("{}/oauth/token", self.base_url))
            .json(&data)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "Status code response: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await.unwrap(),
            ));
        }

        let data: TokenResponse = res.json().await?;
        self.token = Some(data.access_token.clone());

        Ok(())
    }

    pub async fn home_timeline(&self, from_id: Option<&str>) -> Result<Vec<Tweet>> {
        let mut req = self.http.get("{}/api/v1/timelines/home");
        if from_id.is_some() {
            req = req.query(&[("since_id", from_id.unwrap())]);
        }
        let res = req.send().await?;
        let data: Vec<Tweet> = res.json().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use super::Backend;

    #[tokio::test]
    async fn new_backend() {
        let b = Backend::new("https://cawfee.club").await.unwrap();
        assert!(b.credentials.is_some());
    }
}
