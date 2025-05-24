use std::collections::HashMap;

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct CredentialApplication {
    client_id: String,
    client_secret: String,
}

struct Backend {
    base_url: String,
    http: Client,
    credentials: Option<CredentialApplication>,
}

impl Backend {
    async fn new(base_url: &str) -> Result<Self> {
        let mut ris = Backend {
            base_url: base_url.to_string(),
            http: Client::new(),
            credentials: None,
        };

        let mut data = HashMap::new();
        data.insert("client_name", "plerustax");
        data.insert("redirect_uris", "urn:ietf:wg:oauth:2.0:oob");

        let res = ris
            .http
            .post(format!("{}/api/v1/apps", ris.base_url.as_str()))
            .json(&data)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow!("Status code response: {}", res.status().as_u16()));
        }

        let data: CredentialApplication = res.json().await?;
        ris.credentials = Some(data);

        Ok(ris)
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
