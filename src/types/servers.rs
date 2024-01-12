// Copyright (c) Tribufu. All Rights Reserved.

use crate::users::MiniProfile;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub game_port: Option<u16>,
    pub query_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcon_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcon_password: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub package_id: u64,
    pub package_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<ServerPackage>,
    pub version: Option<String>,
    pub cluster_id: Option<u32>,
    pub website_url: Option<String>,
    pub banner_url: Option<String>,
    pub owner_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<MiniProfile>,
    #[serde(skip)]
    pub uptime: f64,
    pub last_online: Option<NaiveDateTime>,
    #[serde(flatten)]
    pub stats: ServerStats,
    pub country: Option<String>,
    pub steam: bool,
    pub discord_server_id: Option<String>,
    pub youtube_video_url: Option<String>,
    pub tags: Option<Value>,
    pub comment_count: u32,
    #[serde(skip)]
    pub secret: Option<String>,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    Unknown,
    Offline,
    Online,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStats {
    pub status: ServerStatus,
    pub ping: Option<u32>,
    pub map: Option<String>,
    pub used_slots: Option<i32>,
    pub max_slots: Option<i32>,
    pub motd: Option<String>,
    pub players: Option<Value>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerPackage {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u64,
    pub name: String,
    pub slug: Option<String>,
    pub rust_gamedig_id: Option<String>,
    pub node_gamedig_id: Option<String>,
    pub server_connect_url: Option<String>,
}
