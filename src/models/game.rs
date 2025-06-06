/*
 * Tribufu API
 *
 * REST API to access Tribufu services.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: contact@tribufu.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "game_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub game_port: Option<Option<i32>>,
    #[serde(rename = "query_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub query_port: Option<Option<i32>>,
    #[serde(rename = "rcon_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rcon_port: Option<Option<i32>>,
    #[serde(rename = "server_count", skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[serde(rename = "steam_app_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub steam_app_id: Option<Option<i32>>,
    #[serde(rename = "steam_server_app_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub steam_server_app_id: Option<Option<i32>>,
    #[serde(rename = "enable_servers", skip_serializing_if = "Option::is_none")]
    pub enable_servers: Option<bool>,
    #[serde(rename = "rust_gamedig_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rust_gamedig_id: Option<Option<String>>,
    #[serde(rename = "node_gamedig_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub node_gamedig_id: Option<Option<String>>,
    #[serde(rename = "server_connect_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_connect_url: Option<Option<String>>,
    #[serde(rename = "server_tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_tags: Option<Option<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::ApplicationType>,
    #[serde(rename = "organization_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<Option<String>>,
    #[serde(rename = "icon_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
    #[serde(rename = "banner_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<Option<String>>,
    #[serde(rename = "capsule_image_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub capsule_image_url: Option<Option<String>>,
    #[serde(rename = "library_image_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub library_image_url: Option<Option<String>>,
    #[serde(rename = "parent_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
    #[serde(rename = "slug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slug: Option<Option<String>>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<i32>,
    #[serde(rename = "password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<i32>,
    #[serde(rename = "user_count", skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    #[serde(rename = "achievement_count", skip_serializing_if = "Option::is_none")]
    pub achievement_count: Option<i32>,
    #[serde(rename = "badge_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub badge_count: Option<Option<i32>>,
    #[serde(rename = "download_count", skip_serializing_if = "Option::is_none")]
    pub download_count: Option<i32>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated: Option<Option<String>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_port: None,
            query_port: None,
            rcon_port: None,
            server_count: None,
            steam_app_id: None,
            steam_server_app_id: None,
            enable_servers: None,
            rust_gamedig_id: None,
            node_gamedig_id: None,
            server_connect_url: None,
            server_tags: None,
            id: None,
            name: None,
            description: None,
            r#type: None,
            organization_id: None,
            icon_url: None,
            banner_url: None,
            capsule_image_url: None,
            library_image_url: None,
            parent_id: None,
            slug: None,
            visibility: None,
            password: None,
            primary: None,
            user_count: None,
            achievement_count: None,
            badge_count: None,
            download_count: None,
            created: None,
            updated: None,
        }
    }
}

