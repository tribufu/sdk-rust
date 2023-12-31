// Copyright (c) Tribufu. All Rights Reserved.

use alnilam_consts::TARGET_TRIPLE;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use mintaka_error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::env;
use tribufu_constants::VERSION;
use tribufu_types::games::Game;
use tribufu_types::oauth2::{OAuth2GrantType, OAuth2TokenRequest, OAuth2TokenResponse};
use tribufu_types::users::*;

pub enum Credentials {
    Anonymous,
    ApiKey {
        api_key: String,
    },
    Client {
        client_id: u64,
        client_secret: String,
    },
}

pub enum Token {
    ApiKey {
        api_key: String,
    },
    Basic {
        basic_token: String,
    },
    Bearer {
        access_token: String,
        refresh_token: Option<String>,
    },
}

pub struct TribufuApi {
    base_url: String,
    credentials: Credentials,
    token: Option<Token>,
    http: Client,
}

impl Default for TribufuApi {
    fn default() -> Self {
        Self::new(Credentials::Anonymous)
    }
}

impl TribufuApi {
    const TRIBUFU_API_URL: &'static str = "https://api.tribufu.com";

    pub fn new(credentials: Credentials) -> Self {
        let http = Client::builder()
            .user_agent(Self::user_agent())
            .default_headers(Self::default_headers())
            .build()
            .unwrap();

        Self {
            base_url: Self::get_base_url(),
            credentials,
            token: None,
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

    fn get_base_url() -> String {
        if cfg!(debug_assertions) {
            return env::var("TRIBUFU_API_URL")
                .unwrap_or_else(|_| Self::TRIBUFU_API_URL.to_string());
        }

        Self::TRIBUFU_API_URL.to_string()
    }

    pub fn with_api_key(api_key: String) -> Self {
        Self::new(Credentials::ApiKey { api_key })
    }

    pub fn with_client(client_id: u64, client_secret: String) -> Self {
        Self::new(Credentials::Client {
            client_id,
            client_secret,
        })
    }

    pub fn with_api_key_from_env() -> Option<Self> {
        if let Ok(api_key) = env::var("TRIBUFU_API_KEY") {
            Some(Self::with_api_key(api_key))
        } else {
            None
        }
    }

    pub fn with_client_from_env() -> Option<Self> {
        let client_id = env::var("TRIBUFU_CLIENT_ID");
        let client_secret = env::var("TRIBUFU_CLIENT_SECRET");

        if let (Ok(client_id), Ok(client_secret)) = (client_id, client_secret) {
            Some(Self::with_client(client_id.parse().unwrap(), client_secret))
        } else {
            None
        }
    }

    pub fn set_anonymous(&mut self) {
        self.credentials = Credentials::Anonymous;
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.credentials = Credentials::ApiKey { api_key };
    }

    pub fn set_clients(&mut self, client_id: u64, client_secret: String) {
        self.credentials = Credentials::Client {
            client_id,
            client_secret,
        };
    }

    pub fn set_basic_token(&mut self, basic_token: String) {
        self.token = Some(Token::Basic { basic_token });
    }

    pub fn set_bearer_token(&mut self, access_token: String, refresh_token: Option<String>) {
        self.token = Some(Token::Bearer {
            access_token,
            refresh_token,
        });
    }

    #[inline]
    fn headers(&self) -> HeaderMap {
        let mut headers = Self::default_headers();

        match &self.token {
            Some(token) => match token {
                Token::ApiKey { api_key } => {
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("ApiKey {}", api_key)).unwrap(),
                    );
                }
                Token::Basic { basic_token } => {
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Basic {}", basic_token)).unwrap(),
                    );
                }
                Token::Bearer { access_token, .. } => {
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
                    );
                }
            },
            None => {}
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
        &self,
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

        Ok(response.json().await?)
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
