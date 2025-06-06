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
pub struct LeaderboardItem {
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "display_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Option<String>>,
    #[serde(rename = "photo_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<Option<String>>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "experience", skip_serializing_if = "Option::is_none")]
    pub experience: Option<f64>,
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<f64>,
}

impl LeaderboardItem {
    pub fn new() -> LeaderboardItem {
        LeaderboardItem {
            name: None,
            display_name: None,
            photo_url: None,
            level: None,
            experience: None,
            points: None,
        }
    }
}

