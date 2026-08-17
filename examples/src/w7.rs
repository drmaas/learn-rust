//! Week 7 — Modules, crates, testing, docs.
//!
//! The plugin system from Week 6 doubles as the Week 7 "library crate"
//! exercise (see `w6::Registry`), with an integration test in `tests/`.
//! Here we cover the serde/serialization example and a small unit-converter
//! library (the Week 7 Day 5 "publishable crate" exercise).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub retries: u32,
}

/// Serialize to JSON.
pub fn serialize_config(cfg: &Config) -> String {
    serde_json::to_string(cfg).unwrap()
}

/// Deserialize from JSON — errors are `Result`.
pub fn deserialize_config(json: &str) -> Result<Config, serde_json::Error> {
    serde_json::from_str(json)
}

/// Week 7 Day 5 practice: a small unit-converter library with a real error
/// type and tests — the "publishable crate" checklist in miniature.
#[derive(Debug, PartialEq)]
pub enum ConvertError {
    UnknownUnit(String),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::UnknownUnit(u) => write!(f, "unknown unit: {u}"),
        }
    }
}

impl std::error::Error for ConvertError {}

/// Convert temperature between C, F, and K.
pub fn convert_temperature(value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
    let celsius = match from {
        "C" => value,
        "F" => (value - 32.0) * 5.0 / 9.0,
        "K" => value - 273.15,
        other => return Err(ConvertError::UnknownUnit(other.to_string())),
    };
    match to {
        "C" => Ok(celsius),
        "F" => Ok(celsius * 9.0 / 5.0 + 32.0),
        "K" => Ok(celsius + 273.15),
        other => Err(ConvertError::UnknownUnit(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let cfg = Config {
            name: "app".into(),
            retries: 3,
        };
        let json = serialize_config(&cfg);
        let back: Config = deserialize_config(&json).unwrap();
        assert_eq!(cfg, back);
    }

    #[test]
    fn deserialize_rejects_bad_json() {
        assert!(deserialize_config("{ not json").is_err());
    }

    #[test]
    fn temperature_conversions() {
        assert_eq!(convert_temperature(0.0, "C", "F"), Ok(32.0));
        assert_eq!(convert_temperature(100.0, "C", "K"), Ok(373.15));
        assert_eq!(convert_temperature(32.0, "F", "C"), Ok(0.0));
        assert!((convert_temperature(300.0, "K", "F").unwrap() - 80.33).abs() < 1e-9);
    }

    #[test]
    fn unknown_unit_errors() {
        assert!(matches!(
            convert_temperature(1.0, "X", "C"),
            Err(ConvertError::UnknownUnit(_))
        ));
        assert!(matches!(
            convert_temperature(1.0, "C", "X"),
            Err(ConvertError::UnknownUnit(_))
        ));
    }
}
