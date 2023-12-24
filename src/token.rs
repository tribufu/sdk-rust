// Copyright (c) Tribufu. All Rights Reserved.

use crate::oauth2::OAuth2TokenResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Credentials {
    ApiKey(String),
    Token(OAuth2TokenResponse),
}
