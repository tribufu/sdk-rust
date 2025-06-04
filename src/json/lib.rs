// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Serializer;
use tribufu_error::Result;

#[macro_export]
macro_rules! include_json {
    ($path:expr) => {
        $crate::from_str(include_str!($path)).expect("Failed to load JSON")
    };
}

pub use serde_json::from_slice;
pub use serde_json::from_str;
pub use serde_json::to_string;
pub use serde_json::to_value;
pub use serde_json::to_vec_pretty;

pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let obj = to_value(value).expect("Failed to serialize JSON");
    let mut buf = Vec::new();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    obj.serialize(&mut ser)?;
    Ok(String::from_utf8(buf)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct JsonTest {
        version: String,
    }

    #[test]
    fn test_include_json() {
        let data: JsonTest = include_json!("test.json");
        assert!(data.version == "0.0.0".to_owned())
    }
}
