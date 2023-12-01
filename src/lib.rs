// Copyright (c) Tribufu. All Rights Reserved.

#![allow(dead_code)]

use alnilam_consts::TARGET_TRIPLE;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct TribufuClient {
    client_id: u64,
    client_secret: String,
    http: Client,
}

impl TribufuClient {
    const BASE_URL: &'static str = "https://api.tribufu.com";

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
