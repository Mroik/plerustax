use std::collections::HashMap;

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Deserialize;

use crate::app::backend::Backend;

use super::{account::Account, tweet::Tweet};

#[derive(Deserialize, Debug)]
struct CredentialApplication {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    accounts: Vec<Account>,
    statuses: Vec<Tweet>,
}

#[derive(Debug)]
pub struct Api {
    base_url: String,
    http: Client,
    credentials: Option<CredentialApplication>,
    token: Option<String>,
}

impl Api {
    pub async fn new(base_url: &str) -> Result<Self> {
        let mut ris = Api {
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
        data.insert("scope", "read write");

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
        self.token = Some(data.access_token);

        Ok(())
    }

    pub async fn home_timeline(&self, from_id: Option<&str>) -> Result<Vec<Tweet>> {
        let mut req = self
            .http
            .get(format!("{}/api/v1/timelines/home", self.base_url))
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap()),
            );
        if from_id.is_some() {
            req = req.query(&[("since_id", from_id.unwrap())]);
        }
        let res = req.send().await?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "Status: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await?
            ));
        }

        let data: Vec<Tweet> = res.json().await?;
        Ok(data)
    }

    pub async fn public_timeline(&self, from_id: Option<&str>) -> Result<Vec<Tweet>> {
        let mut req = self
            .http
            .get(format!("{}/api/v1/timelines/public", self.base_url))
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap()),
            );
        if from_id.is_some() {
            req = req.query(&[("since_id", from_id.unwrap())]);
        }
        let res = req.send().await?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "Status: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await?
            ));
        }

        let data: Vec<Tweet> = res.json().await?;
        Ok(data)
    }

    pub async fn search_tweets(
        &self,
        search_term: &str,
        offset: Option<u32>,
    ) -> Result<SearchResult> {
        let mut queries = vec![("q", search_term)];
        let offset_str = offset.as_ref().unwrap_or(&0).to_string();
        if !offset.is_some() {
            queries.push(("offset", &offset_str));
        }
        let req = self
            .http
            .get(format!("{}/api/v2/search", self.base_url))
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap()),
            )
            .query(&queries);
        let res = req.send().await?;

        if !res.status().is_success() {
            return Err(anyhow!(
                "Status: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await?
            ));
        }

        let data: SearchResult = res.json().await?;
        Ok(data)
    }

    pub async fn post_tweet(&self, text: &str, visibility: &str) -> Result<()> {
        let req = self
            .http
            .post(format!("{}/api/v1/statuses", self.base_url))
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap()),
            );
        let mut params = HashMap::new();
        params.insert("status", text);
        params.insert("content_type", "text/plain");
        params.insert("source", "plerustax");
        params.insert("visibility", visibility);

        let res = req.json(&params).send().await?;
        if !res.status().is_success() {
            return Err(anyhow!(
                "Status: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await?
            ));
        }
        Ok(())
    }

    pub async fn delete_tweet(&self, id: &str) -> Result<()> {
        let res = self
            .http
            .delete(format!("{}/api/v1/statuses/{}", self.base_url, id))
            .header(
                "Authorization",
                format!("Bearer {}", self.token.as_ref().unwrap()),
            )
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(anyhow!(
                "Status: {}\nMessage: {}",
                res.status().as_u16(),
                res.text().await?
            ));
        }
        Ok(())
    }

    pub async fn backend(self) -> Backend {
        Backend::new(self).await
    }
}

#[cfg(test)]
mod test {
    use super::Api;

    #[tokio::test]
    async fn new_backend() {
        let b = Api::new("https://cawfee.club").await.unwrap();
        assert!(b.credentials.is_some());
    }
}
