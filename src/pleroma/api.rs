use std::collections::HashMap;

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Deserialize;

use super::tweet::Tweet;

#[derive(Deserialize, Debug)]
struct CredentialApplication {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
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
        let mut req = self
            .http
            .get(format!("{}/api/v1/timelines/home", self.base_url));
        if from_id.is_some() {
            req = req.query(&[("since_id", from_id.unwrap())]);
        }
        let res = req
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

        let data: Vec<Tweet> = res.json().await?;
        Ok(data)
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
