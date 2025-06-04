// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use std::env;
use std::path::PathBuf;
use tribufu_error::Result;

/// Gets the root base directory of the application.
pub fn app_dir() -> Result<PathBuf> {
    let mut path = dunce::canonicalize(env::current_exe()?)?; // /bin/platform/app.exe
    path.pop(); // /bin
    path.pop(); // /
    Ok(path)
}

/// Gets the path to the platform-specific binary directory.
pub fn bin_dir() -> Result<PathBuf> {
    let base_dir = app_dir()?;

    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    return Ok(base_dir.join("MacOS"));

    #[cfg(not(all(target_os = "macos", not(debug_assertions))))]
    Ok(base_dir.join("bin"))
}

/// Gets the path to the configuration directory.
pub fn config_dir() -> Result<PathBuf> {
    Ok(app_dir()?.join("config"))
}

/// Gets the path to the assets directory.
pub fn assets_dir() -> Result<PathBuf> {
    Ok(app_dir()?.join("assets"))
}

/// Gets the path to the saved data directory.
pub fn saved_dir() -> Result<PathBuf> {
    Ok(app_dir()?.join("saved"))
}

/// Gets the path to the cache directory inside `saved`.
pub fn cache_dir() -> Result<PathBuf> {
    Ok(saved_dir()?.join("cache"))
}

/// Gets the path to the logs directory inside `saved`.
pub fn logs_dir() -> Result<PathBuf> {
    Ok(saved_dir()?.join("logs"))
}
