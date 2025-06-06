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
pub struct SearchRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::SearchType>,
    #[serde(rename = "query", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub query: Option<Option<String>>,
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i32>>,
    #[serde(rename = "game_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<Option<String>>,
}

impl SearchRequest {
    pub fn new() -> SearchRequest {
        SearchRequest {
            r#type: None,
            query: None,
            page: None,
            game_id: None,
        }
    }
}

