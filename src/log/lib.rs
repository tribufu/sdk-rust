// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use chrono::prelude::*;
pub use env_logger::fmt::Color;
use env_logger::{Builder, Target};
pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::trace;
pub use log::warn;
use log::Level;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod colors;

mod log_config;
pub use log_config::*;

pub fn init() {
    let logger = Logger::from_env(None);
    logger.init();
}

pub fn init_level(level: LogLevel) {
    let logger = Logger::new(level.into());
    logger.init();
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "off" => Some(Self::Off),
            "error" => Some(Self::Error),
            "warn" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            "trace" => Some(Self::Trace),
            _ => None,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

pub struct Logger {
    builder: Builder,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        let mut builder = Builder::new();
        builder.filter_level(level);
        Self { builder }
    }

    pub fn with_config(config: LogConfig) -> Self {
        let mut builder = Builder::new();
        builder.filter_level(config.level.into());
        Self { builder }
    }

    pub fn from_env(var: Option<String>) -> Self {
        let builder = Builder::from_env(var.unwrap_or("LOG_LEVEL".to_string()));
        Self { builder }
    }

    pub fn init(mut self) {
        self.builder
            .target(Target::Stdout)
            .format(|fmt, record| {
                let mut style = fmt.style();

                match record.level() {
                    Level::Error => style.set_color(Color::Ansi256(colors::RED)),
                    Level::Warn => style.set_color(Color::Ansi256(colors::YELLOW)),
                    Level::Info => style.set_color(Color::Ansi256(colors::GREEN)),
                    Level::Debug => style.set_color(Color::Ansi256(colors::WHITE)),
                    Level::Trace => style.set_color(Color::Ansi256(colors::BRIGHT_BLACK)),
                };

                let line = format!(
                    "[{}] [{}]: {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.level(),
                    record.args()
                );

                writeln!(fmt, "{}", style.value(line))
            })
            .init();
    }
}
