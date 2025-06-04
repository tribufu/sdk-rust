// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use std::env::consts;

pub const BUILD_TIMESTAMP: &'static str = env!("VERGEN_BUILD_TIMESTAMP");
pub const CARGO_PROFILE: &'static str = env!("VERGEN_CARGO_PROFILE");
pub const LLVM_VERSION: &'static str = env!("VERGEN_RUSTC_LLVM_VERSION");
pub const RUSTC_CHANNEL: &'static str = env!("VERGEN_RUSTC_CHANNEL");
pub const RUSTC_COMMIT: &'static str = env!("VERGEN_RUSTC_COMMIT_HASH");
pub const RUSTC_VERSION: &'static str = env!("VERGEN_RUSTC_SEMVER");
pub const TARGET_ARCH: &'static str = consts::ARCH;
pub const TARGET_DLL_EXTENSION: &'static str = consts::DLL_EXTENSION;
pub const TARGET_DLL_SUFFIX: &'static str = consts::DLL_SUFFIX;
pub const TARGET_EXE_EXTENSION: &'static str = consts::EXE_EXTENSION;
pub const TARGET_EXE_SUFFIX: &'static str = consts::EXE_SUFFIX;
pub const TARGET_FAMILY: &'static str = consts::FAMILY;
pub const TARGET_OS: &'static str = consts::OS;
pub const TARGET_TRIPLE: &'static str = env!("VERGEN_CARGO_TARGET_TRIPLE");
