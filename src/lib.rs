// Copyright (c) Tribufu. All Rights Reserved.

#![allow(dead_code)]

use alnilam_consts::TARGET_TRIPLE;
use anyhow::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod oauth2;
pub mod token;

#[derive(Clone)]
pub struct TribufuClient {
    client_id: u64,
    client_secret: String,
    http: Client,
}

impl TribufuClient {
    const BASE_URL: &'static str = "http://localhost:5000";

    pub fn new(id: u64, secret: impl Into<String>) -> Result<TribufuClient> {
        let user_agent = format!(
            "Tribufu/{} (+https://api.tribufu.com; {})",
            VERSION, TARGET_TRIPLE
        );

        let mut headers = HeaderMap::new();
        headers.insert("X-Tribufu-Language", HeaderValue::from_static("rust"));
        headers.insert("X-Tribufu-Version", HeaderValue::from_static(VERSION));

        let http = Client::builder()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(TribufuClient {
            client_id: id,
            client_secret: secret.into(),
            http,
        })
    }

    pub fn id(&self) -> u64 {
        self.client_id
    }

    pub async fn get_token(&self) -> Result<oauth2::OAuth2TokenResponse> {
        let body = oauth2::OAuth2TokenRequest {
            grant_type: oauth2::OAuth2GrantType::ClientCredentials,
            code: None,
            refresh_token: None,
            username: None,
            password: None,
            client_id: Some(self.client_id.to_string()),
            client_secret: Some(self.client_secret.clone()),
            redirect_uri: None,
        };

        let response = match self
            .http
            .post(format!("{}/v1/oauth2/token", Self::BASE_URL))
            .form(&body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };

        if response.status() != 200 {
            return Err(Error::msg(format!(
                "Failed to get token: {}",
                response.status()
            )));
        }

        let token = response.json::<oauth2::OAuth2TokenResponse>().await?;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client() {
        let client = TribufuClient::new(0, "client_secret").unwrap();
        assert_eq!(client.id(), 0);
    }
}
