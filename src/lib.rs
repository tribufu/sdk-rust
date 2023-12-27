// Copyright (c) Tribufu. All Rights Reserved.

#![allow(dead_code)]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod api;
pub mod games;
pub mod oauth2;
pub mod users;

pub use api::*;
