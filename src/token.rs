// Copyright (c) Tribufu. All Rights Reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    User,
    Bot,
    Client,
    Server,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationType {
    ApiKey,
    Basic,
    Bearer,
}
