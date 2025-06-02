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
