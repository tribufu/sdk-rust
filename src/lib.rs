// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::apis::configuration::{ApiKey, Configuration};
use crate::apis::tribufu_generated_api::TribufuGeneratedApiClient;
use reqwest::Client;
use std::env::{self, consts};
use std::sync::Arc;

pub mod apis;
pub mod models;

/// Use this to interact with the Tribufu API.
pub struct TribufuApi;

impl TribufuApi {
    /// The default base URL for the Tribufu API.
    pub const DEFAULT_BASE_URL: &'static str = "https://api.tribufu.com";

    /// Create a TribufuApi instance.
    pub fn new(api_key: Option<String>) -> TribufuGeneratedApiClient {
        let configuration = Self::create_configuration(api_key);
        let configuration_arc = Arc::new(configuration);
        TribufuGeneratedApiClient::new(configuration_arc)
    }

    /// Create a TribufuApi with the default options.
    pub fn default() -> TribufuGeneratedApiClient {
        Self::new(None)
    }

    /// Create a TribufuApi with the given API key.
    ///
    /// An API key gives you public read only access to the Tribufu API.
    pub fn with_api_key(api_key: String) -> TribufuGeneratedApiClient {
        Self::new(Some(api_key))
    }

    /// Try to create a TribufuApi from environment variables.
    ///
    /// This will only work if the environment variables are set.
    pub fn from_env(prefix: Option<&str>) -> Option<TribufuGeneratedApiClient> {
        let prefix = prefix.unwrap_or("TRIBUFU");
        let api_key_var = format!("{}_API_KEY", prefix);
        if let Ok(api_key) = env::var(api_key_var) {
            if !api_key.trim().is_empty() {
                return Some(Self::with_api_key(api_key));
            }
        }
        None
    }

    /// Create a TribufuApi from environment variables or the default API.
    ///
    /// This will fallback to the default API if the environment variables are not set.
    pub fn from_env_or_default(prefix: Option<&str>) -> TribufuGeneratedApiClient {
        Self::from_env(prefix).unwrap_or_else(Self::default)
    }

    /// Gets the version of the Tribufu API client.
    pub fn get_version() -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    /// Gets the user agent string for the Tribufu API client.
    pub fn get_user_agent() -> String {
        let version = Self::get_version();
        format!("Tribufu/{} ({}; {})", version, consts::OS, consts::ARCH)
    }

    /// Checks if debug mode is enabled.
    pub fn debug_enabled() -> bool {
        cfg!(debug_assertions)
    }

    /// Get the base URL for the Tribufu API.
    fn get_base_url() -> String {
        if let Ok(base_url) = env::var("TRIBUFU_API_URL") {
            if Self::debug_enabled() && !base_url.trim().is_empty() {
                return base_url;
            }
        }

        Self::DEFAULT_BASE_URL.to_string()
    }

    /// Creates a configuration for the Tribufu API client.
    fn create_configuration(api_key: Option<String>) -> Configuration {
        let base_path = Self::get_base_url();
        let user_agent = Some(Self::get_user_agent());

        let api_key_obj = if let Some(api_key) = api_key {
            Some(ApiKey {
                prefix: Some("ApiKey".to_owned()),
                key: api_key,
            })
        } else {
            None
        };

        Configuration {
            base_path,
            user_agent,
            api_key: api_key_obj,
            ..Default::default()
        }
    }
}
