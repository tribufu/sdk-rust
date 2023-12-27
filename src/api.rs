// Copyright (c) Tribufu. All Rights Reserved.

use crate::games::Game;
use crate::oauth2::{OAuth2GrantType, OAuth2TokenRequest, OAuth2TokenResponse};
use crate::users::*;
use crate::VERSION;
use alnilam_consts::TARGET_TRIPLE;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use chrono::{NaiveDateTime, Utc};
use mintaka_error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::env;

pub enum CredentialsType {
    Anonymous,
    ApiKey,
    Basic,
    Bearer,
}

pub struct TribufuApi {
    base_url: String,
    credentials: Option<String>,
    credentials_kind: CredentialsType,
    credentials_refreshed_at: Option<NaiveDateTime>,
    credentials_expires_at: Option<NaiveDateTime>,
    http: Client,
}

impl Default for TribufuApi {
    fn default() -> Self {
        Self::new(CredentialsType::Anonymous, None)
    }
}

impl TribufuApi {
    const BASE_URL: &'static str = "https://api.tribufu.com";

    pub fn new(credentials_kind: CredentialsType, credentials: Option<String>) -> Self {
        let http = Client::builder()
            .user_agent(Self::user_agent())
            .default_headers(Self::default_headers())
            .build()
            .unwrap();

        Self {
            base_url: Self::BASE_URL.to_string(),
            credentials,
            credentials_kind,
            credentials_refreshed_at: None,
            credentials_expires_at: None,
            http,
        }
    }

    pub fn debug_enabled(&self) -> bool {
        return cfg!(debug_assertions);
    }

    #[inline]
    fn user_agent() -> String {
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

    pub fn with_api_key(api_key: String) -> Self {
        let mut api = Self::default();
        api.use_api_key(api_key);
        api
    }

    pub fn with_client(client_id: u64, client_secret: String) -> Self {
        let mut api = Self::default();
        api.use_client(client_id, client_secret);
        api
    }

    pub fn with_token(token: String) -> Self {
        let mut api = Self::default();
        api.use_token(token);
        api
    }

    pub fn from_env() -> Self {
        let mut api = Self::default();
        api.use_env();
        api
    }

    pub fn use_env(&mut self) {
        #[cfg(debug_assertions)]
        if let Ok(base_url) = env::var("TRIBUFU_API_URL") {
            self.set_base_url(base_url);
        }

        if let Ok(api_key) = env::var("TRIBUFU_API_KEY") {
            self.use_api_key(api_key);
        }

        let client_id = env::var("TRIBUFU_CLIENT_ID");
        let client_secret = env::var("TRIBUFU_CLIENT_SECRET");

        if let (Ok(client_id), Ok(client_secret)) = (client_id, client_secret) {
            self.use_client(client_id.parse().unwrap(), client_secret);
        }

        if let Ok(token) = env::var("TRIBUFU_TOKEN") {
            self.use_token(token);
        }
    }

    pub fn use_anonymous(&mut self) {
        self.credentials_kind = CredentialsType::Anonymous;
        self.credentials = None;
    }

    pub fn use_api_key(&mut self, api_key: String) {
        self.credentials_kind = CredentialsType::ApiKey;
        self.credentials = Some(api_key);
    }

    pub fn use_client(&mut self, client_id: u64, client_secret: String) {
        let credentials_str = format!("{}:{}", client_id, client_secret);
        self.credentials_kind = CredentialsType::Basic;
        self.credentials = Some(BASE64.encode(credentials_str.as_bytes()));
    }

    pub fn use_token(&mut self, token: String) {
        self.credentials_kind = CredentialsType::Bearer;
        self.credentials = Some(token);
    }

    fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    #[inline]
    fn headers(&self) -> HeaderMap {
        let mut headers = Self::default_headers();

        match self.credentials_kind {
            CredentialsType::ApiKey => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!(
                        "ApiKey {}",
                        self.credentials.as_ref().unwrap()
                    ))
                    .unwrap(),
                );
            }
            CredentialsType::Basic => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Basic {}", self.credentials.as_ref().unwrap()))
                        .unwrap(),
                );
            }
            CredentialsType::Bearer => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!(
                        "Bearer {}",
                        self.credentials.as_ref().unwrap()
                    ))
                    .unwrap(),
                );
            }
            _ => {}
        }

        headers
    }

    pub async fn get_token_with_code(
        &mut self,
        code: String,
        client_id: u64,
        client_secret: String,
    ) -> Result<OAuth2TokenResponse> {
        self.get_oauth_token(
            OAuth2GrantType::AuthorizationCode,
            Some(code),
            client_id,
            client_secret,
            None,
            None,
        )
        .await
    }

    pub async fn get_token_from_password(
        &mut self,
        username: String,
        password: String,
        client_id: u64,
        client_secret: String,
    ) -> Result<OAuth2TokenResponse> {
        self.get_oauth_token(
            OAuth2GrantType::Password,
            Some(password),
            client_id,
            client_secret,
            None,
            Some(username),
        )
        .await
    }

    pub async fn get_token_from_passkey(
        &mut self,
        username: String,
        passkey: String,
        client_id: u64,
        client_secret: String,
    ) -> Result<OAuth2TokenResponse> {
        self.get_oauth_token(
            OAuth2GrantType::Passkey,
            Some(passkey),
            client_id,
            client_secret,
            None,
            Some(username),
        )
        .await
    }

    pub async fn refresh_token(
        &mut self,
        refresh_token: String,
        client_id: u64,
        client_secret: String,
    ) -> Result<OAuth2TokenResponse> {
        self.get_oauth_token(
            OAuth2GrantType::RefreshToken,
            Some(refresh_token),
            client_id,
            client_secret,
            None,
            None,
        )
        .await
    }

    pub async fn get_client_token(
        &mut self,
        client_id: u64,
        client_secret: String,
    ) -> Result<OAuth2TokenResponse> {
        self.get_oauth_token(
            OAuth2GrantType::ClientCredentials,
            None,
            client_id,
            client_secret,
            None,
            None,
        )
        .await
    }

    pub async fn get_server_token(
        &mut self,
        server_id: u64,
        client_id: u64,
        client_secret: String,
    ) -> Result<OAuth2TokenResponse> {
        self.get_oauth_token(
            OAuth2GrantType::ClientCredentials,
            None,
            client_id,
            client_secret,
            Some("server_id".to_string()),
            Some(server_id.to_string()),
        )
        .await
    }

    async fn get_oauth_token(
        &mut self,
        grant_type: OAuth2GrantType,
        grant_value: Option<String>,
        client_id: u64,
        client_secret: String,
        subject_key: Option<String>,
        subject_value: Option<String>,
    ) -> Result<OAuth2TokenResponse> {
        let code = if grant_type == OAuth2GrantType::AuthorizationCode {
            grant_value.clone()
        } else {
            None
        };

        let refresh_token = if grant_type == OAuth2GrantType::RefreshToken {
            grant_value.clone()
        } else {
            None
        };

        let mut require_username = false;

        let password = if grant_type == OAuth2GrantType::Password {
            require_username = true;
            grant_value.clone()
        } else {
            None
        };

        let passkey = if grant_type == OAuth2GrantType::Passkey {
            require_username = true;
            grant_value.clone()
        } else {
            None
        };

        let username = if require_username && subject_value.is_some() {
            subject_value.clone()
        } else {
            None
        };

        let request_body = OAuth2TokenRequest {
            grant_type,
            code,
            refresh_token,
            username,
            password,
            passkey,
            client_id: Some(client_id.to_string()),
            client_secret: Some(client_secret.clone()),
            redirect_uri: None,
        };

        let params = if subject_key.is_some() && subject_value.is_some() {
            format!("?{}={}", subject_key.unwrap(), subject_value.unwrap())
        } else {
            "".to_string()
        };

        let url = format!("{}/v1/oauth2/token{}", self.base_url, params);
        let headers = self.headers();
        let response = self
            .http
            .post(url)
            .headers(headers)
            .form(&request_body)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(Error::msg(format!(
                "Failed to get token: {}",
                response.status()
            )));
        }

        let response_body: OAuth2TokenResponse = response.json().await?;

        self.use_token(response_body.clone().access_token);

        Ok(response_body)
    }

    pub async fn get_user_info(&self) -> Result<User> {
        let url = format!("{}/v1/oauth2/userinfo", self.base_url);
        let headers = self.headers();
        let response = self.http.get(url).headers(headers).send().await?;

        Ok(response.json().await?)
    }

    pub async fn get_games(&self, page: Option<u32>) -> Result<Vec<Game>> {
        let page = page.unwrap_or(1);
        let url = format!("{}/v1/packages?page={}", self.base_url, page);
        let headers = self.headers();
        let response = self.http.get(url).headers(headers).send().await?;

        Ok(response.json().await?)
    }

    pub async fn get_game(&self, id: u64) -> Result<Game> {
        let url = format!("{}/v1/packages/{}", self.base_url, id);
        let headers = self.headers();
        let response = self.http.get(url).headers(headers).send().await?;

        Ok(response.json().await?)
    }
}
