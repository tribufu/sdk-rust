// Copyright (c) Tribufu. All Rights Reserved.

use std::env;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client as HttpClient,
};
use tribufu_constants::{TRIBUFU_API_URL, TRIBUFU_VERSION};

#[derive(Debug, Default)]
pub enum TribufuApiOptions {
    #[default]
    Anonymous,
    ApiKey {
        api_key: String,
    },
    Token {
        access_token: String,
        refresh_token: Option<String>,
        expires_in: Option<u64>,
    },
}

/// **Tribufu API**
///
/// Use this class to interact with the Tribufu API.
///
/// *There are three ways to use the Tribufu API:*
/// - A api key give you public read only access to the Tribufu API.
/// - A bot give you read and write access to the Tribufu API as a bot account.
/// - A client give you read and write access to the Tribufu API as a client application.
pub struct TribufuApi {
    pub(crate) http: HttpClient,
    pub(crate) options: TribufuApiOptions,
}

impl Default for TribufuApi {
    fn default() -> Self {
        Self::new(TribufuApiOptions::Anonymous)
    }
}

impl TribufuApi {
    pub fn new(options: TribufuApiOptions) -> Self {
        let http = HttpClient::builder()
            .default_headers(Self::default_headers())
            .build()
            .unwrap();

        TribufuApi { http, options }
    }

    pub fn with_api_key(api_key: String) -> Self {
        Self::new(TribufuApiOptions::ApiKey { api_key })
    }

    pub fn from_env(prefix: Option<&str>) -> Option<Self> {
        let prefix = if let Some(prefix) = prefix {
            format!("{}_", prefix)
        } else {
            "".to_string()
        };

        if let Ok(api_key) = env::var(format!("{prefix}API_KEY")) {
            Some(Self::with_api_key(api_key))
        } else {
            None
        }
    }

    pub fn from_env_or_default(prefix: Option<&str>) -> Self {
        Self::from_env(prefix).unwrap_or_default()
    }

    pub(crate) fn debug_enabled() -> bool {
        return cfg!(debug_assertions);
    }

    #[inline]
    pub(crate) fn get_base_url() -> String {
        if Self::debug_enabled() {
            return env::var("TRIBUFU_API_URL").unwrap_or(TRIBUFU_API_URL.to_string());
        }

        TRIBUFU_API_URL.to_string()
    }

    #[inline]
    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tribufu-Language", "rust".parse().unwrap());
        headers.insert("X-Tribufu-Version", TRIBUFU_VERSION.parse().unwrap());
        headers
    }

    pub(crate) fn headers(&self) -> HeaderMap {
        let mut headers = TribufuApi::default_headers();

        match &self.options {
            TribufuApiOptions::ApiKey { api_key } => {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!("ApiKey {}", api_key)).unwrap(),
                );
            }
            TribufuApiOptions::Token { access_token, .. } => {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
                );
            }
            _ => {}
        }

        headers
    }
}
