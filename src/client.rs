// Copyright (c) Tribufu. All Rights Reserved.

use crate::games::Game;
use crate::oauth2::*;
use crate::VERSION;
use alnilam_consts::TARGET_TRIPLE;
use anyhow::{Error, Result};
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

#[derive(Clone)]
pub struct TribufuClient {
    client_id: u64,
    client_secret: String,
    token: Option<OAuth2TokenResponse>,
}

impl TribufuClient {
    //const BASE_URL: &'static str = "https://api.tribufu.com";
    const BASE_URL: &'static str = "http://localhost:5000";

    pub fn new(id: u64, secret: impl Into<String>) -> Result<TribufuClient> {
        Ok(TribufuClient {
            client_id: id,
            client_secret: secret.into(),
            token: None,
        })
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.client_id
    }

    #[inline]
    pub fn user_agent() -> String {
        format!(
            "Tribufu/{} (+https://api.tribufu.com; {})",
            VERSION, TARGET_TRIPLE
        )
    }

    #[inline]
    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tribufu-Language", HeaderValue::from_static("rust"));
        headers.insert("X-Tribufu-Version", HeaderValue::from_static(VERSION));
        headers
    }

    fn http_client(&self) -> Result<Client> {
        let user_agent = Self::user_agent();
        let mut headers = Self::default_headers();

        if let Some(token) = &self.token {
            headers.insert(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token.access_token))?,
            );
        }

        let http = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()?;

        Ok(http)
    }

    pub async fn get_token(&mut self, server_id: Option<u64>) -> Result<()> {
        let server_id = if let Some(server_id) = server_id {
            Some(server_id.to_string())
        } else {
            None
        };

        let body = OAuth2TokenRequest {
            grant_type: OAuth2GrantType::ClientCredentials,
            code: None,
            refresh_token: None,
            username: None,
            password: None,
            client_id: Some(self.client_id.to_string()),
            client_secret: Some(self.client_secret.clone()),
            redirect_uri: None,
            server_id,
        };

        let url = format!("{}/v1/oauth2/token", Self::BASE_URL);
        let response = self.http_client()?.post(url).form(&body).send().await?;

        if response.status() != 200 {
            return Err(Error::msg(format!(
                "Failed to get token: {}",
                response.status()
            )));
        }

        self.token = Some(response.json().await?);

        Ok(())
    }

    pub async fn refresh_token_token(&mut self) -> Result<()> {
        let token = if let Some(token) = &self.token {
            token
        } else {
            return Err(Error::msg(
                format!("Failed to refresh: self.token == None",),
            ));
        };

        if token.refresh_token.is_none() {
            return Err(Error::msg(format!(
                "Failed to refresh: self.token.refresh_token == None",
            )));
        }

        let body = OAuth2TokenRequest {
            grant_type: OAuth2GrantType::RefreshToken,
            code: None,
            refresh_token: token.refresh_token.clone(),
            username: None,
            password: None,
            client_id: Some(self.client_id.to_string()),
            client_secret: Some(self.client_secret.clone()),
            redirect_uri: None,
            server_id: None,
        };

        let url = format!("{}/v1/oauth2/token", Self::BASE_URL);
        let response = self.http_client()?.post(url).form(&body).send().await?;

        if response.status() != 200 {
            return Err(Error::msg(format!(
                "Failed to get token: {}",
                response.status()
            )));
        }

        self.token = Some(response.json().await?);

        Ok(())
    }

    pub async fn get_games(&self) -> Result<Vec<Game>> {
        let url = format!("{}/v1/packages", Self::BASE_URL);
        let response = self.http_client()?.get(url).send().await?;

        Ok(response.json().await?)
    }

    pub async fn get_game(&self, id: u64) -> Result<Game> {
        let url = format!("{}/v1/packages/{}", Self::BASE_URL, id);
        let response = self.http_client()?.get(url).send().await?;

        Ok(response.json().await?)
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
