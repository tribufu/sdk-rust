// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use anyhow::Error;
pub use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;
