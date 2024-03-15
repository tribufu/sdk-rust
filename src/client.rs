// Copyright (c) Tribufu. All Rights Reserved.

use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::{TribufuApi, TribufuApiOptions};

pub struct TribufuClient {
    client_id: u64,
    client_secret: String,
    api: TribufuApi,
}

impl TribufuClient {
    pub fn new(client_id: u64, client_secret: impl Into<String>) -> Self {
        Self {
            client_id,
            client_secret: client_secret.into(),
            api: TribufuApi::default(),
        }
    }

    pub fn from_env(prefix: Option<&str>) -> Option<Self> {
        let prefix = if let Some(prefix) = prefix {
            format!("{}_", prefix)
        } else {
            "".to_string()
        };

        let client_id = env::var(format!("{prefix}CLIENT_ID"));
        let client_secret = env::var(format!("{prefix}CLIENT_SECRET"));

        if let (Ok(client_id), Ok(client_secret)) = (client_id, client_secret) {
            Some(Self::new(client_id.parse().unwrap(), client_secret))
        } else {
            None
        }
    }

    pub(crate) fn set_tokens(
        &mut self,
        access_token: String,
        refresh_token: Option<String>,
        expires_in: Option<u64>,
    ) {
        self.api.options = TribufuApiOptions::Token {
            access_token,
            refresh_token,
            expires_in,
        };
    }

    pub(crate) fn clear_tokens(&mut self) {
        self.api.options = TribufuApiOptions::Anonymous;
    }

    #[inline]
    fn oauth_headers(&self) -> HeaderMap {
        let mut headers = self.api.headers();

        headers.insert("Authorization", HeaderValue::from_str("Basic").unwrap());

        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );

        headers
    }

    pub fn api(&self) -> &TribufuApi {
        &self.api
    }

    pub fn api_mut(&mut self) -> &mut TribufuApi {
        &mut self.api
    }

    pub fn client_id(&self) -> u64 {
        self.client_id
    }
}
