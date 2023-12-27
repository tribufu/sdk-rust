// Copyright (c) Tribufu. All Rights Reserved.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub capsule_image_url: Option<String>,
    pub library_image_url: Option<String>,
    pub slug: Option<String>,
    pub game_port: Option<u16>,
    pub query_port: Option<u16>,
    pub rcon_port: Option<u16>,
    pub steam_app_id: Option<u32>,
    pub steam_server_app_id: Option<u32>,
    pub rust_gamedig_id: Option<String>,
    pub node_gamedig_id: Option<String>,
    pub server_connect_url: Option<String>,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
}
