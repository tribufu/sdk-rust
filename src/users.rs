// Copyright (c) Tribufu. All Rights Reserved.

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    User = 0,
    Bot = 1,
    Org = 2,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub kind: UserType,
    pub public_flags: u64,
    pub verified: bool,
    pub level: u32,
    pub experience: f64,
    pub public_birthday: bool,
    pub birthday: Option<NaiveDate>,
    pub points: f64,
    pub location: Option<String>,
    pub photo_url: Option<String>,
    pub banner_url: Option<String>,
    pub last_online: Option<NaiveDateTime>,
    pub biography: Option<String>,
    pub view_count: u32,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
}
