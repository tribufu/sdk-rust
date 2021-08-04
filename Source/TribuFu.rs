// Copyright (c) TribuFu. All Rights Reserved

//! TribuFu SDK.

#![allow(non_snake_case)]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn Hello(input: String) -> String {
    return format!("Hello {}", input);
}
