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
pub struct Subscription {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "image_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<Option<String>>,
    #[serde(rename = "prices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prices: Option<Option<std::collections::HashMap<String, f64>>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated: Option<Option<String>>,
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription {
            id: None,
            name: None,
            description: None,
            image_url: None,
            prices: None,
            created: None,
            updated: None,
        }
    }
}

