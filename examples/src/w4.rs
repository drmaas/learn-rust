//! Week 4 — Error handling.

use std::fmt;
use std::num::ParseIntError;

/// Errors are return values, not exceptions.
pub fn parse_num(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>()
}

/// `?` propagates the error early.
pub fn add_parsed(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a = parse_num(a)?;
    let b = parse_num(b)?;
    Ok(a + b)
}

/// A custom error type: an enum of everything that can go wrong.
/// (`std::io::Error` is not `PartialEq`, so this enum can't derive it —
/// tests compare it with `matches!` instead.)
#[derive(Debug)]
pub enum ConfigError {
    MissingField(&'static str),
    InvalidNumber(String),
    Io(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(name) => write!(f, "missing field: {name}"),
            ConfigError::InvalidNumber(v) => write!(f, "invalid number: '{v}'"),
            ConfigError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            _ => None,
        }
    }
}

/// `?` uses `From` to convert an `io::Error` into our `ConfigError`.
impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

/// Parse a mini config file. Handles missing host, invalid port, and I/O.
pub fn parse_config(input: &str) -> Result<Config, ConfigError> {
    let mut host: Option<String> = None;
    let mut port: Option<u16> = None;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "host" => host = Some(value.trim().to_string()),
            "port" => {
                let p = value
                    .trim()
                    .parse::<u16>()
                    .map_err(|_| ConfigError::InvalidNumber(value.trim().to_string()))?;
                port = Some(p);
            }
            _ => {}
        }
    }
    Ok(Config {
        host: host.ok_or(ConfigError::MissingField("host"))?,
        port: port.ok_or(ConfigError::MissingField("port"))?,
    })
}

/// Nested `match` version from the course — deliberately verbose, to contrast
/// with the combinator version below.
#[allow(clippy::manual_unwrap_or_default, clippy::manual_unwrap_or)]
pub fn parse_option_match(opt: Option<&str>) -> i32 {
    match opt {
        Some(s) => match s.parse::<i32>() {
            Ok(v) => v,
            Err(_) => 0,
        },
        None => 0,
    }
}

/// ...and the combinator version — they must agree.
pub fn parse_option_combinators(opt: Option<&str>) -> i32 {
    opt.and_then(|s| s.parse().ok()).unwrap_or(0)
}

/// Recoverable failure -> `Result`.
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// A panic version of divide — for when a broken invariant means "stop".
pub fn divide_unchecked(a: f64, b: f64) -> f64 {
    assert!(b != 0.0, "division by zero");
    a / b
}

/// word_count CLI core (Week 4 Day 5).
#[derive(Debug)]
pub enum CliError {
    NoFilename,
    Io(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::NoFilename => write!(f, "usage: word_count <filename>"),
            CliError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}

pub struct WordCount {
    pub words: usize,
    pub lines: usize,
    pub chars: usize,
}

pub fn count_text(text: &str) -> WordCount {
    WordCount {
        words: text.split_whitespace().count(),
        lines: text.lines().count(),
        chars: text.chars().count(),
    }
}

pub fn word_count_from_file(path: &str) -> Result<WordCount, CliError> {
    let text = std::fs::read_to_string(path)?;
    Ok(count_text(&text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num_ok_and_err() {
        assert_eq!(parse_num("42"), Ok(42));
        assert!(parse_num("not a number").is_err());
    }

    #[test]
    fn question_mark_propagates() {
        assert_eq!(add_parsed("42", "17"), Ok(59));
        assert!(add_parsed("42", "nope").is_err());
    }

    #[test]
    fn config_error_displays() {
        assert_eq!(
            ConfigError::MissingField("host").to_string(),
            "missing field: host"
        );
        assert_eq!(
            ConfigError::InvalidNumber("abc".into()).to_string(),
            "invalid number: 'abc'"
        );
    }

    #[test]
    fn parse_config_good() {
        let cfg = parse_config("host=localhost\nport=8080").unwrap();
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 8080);
    }

    #[test]
    fn parse_config_missing_host() {
        assert!(matches!(
            parse_config("port=8080"),
            Err(ConfigError::MissingField("host"))
        ));
    }

    #[test]
    fn parse_config_invalid_port() {
        assert!(matches!(
            parse_config("host=x\nport=abc"),
            Err(ConfigError::InvalidNumber(_))
        ));
    }

    #[test]
    fn combinators_agree_with_match() {
        assert_eq!(parse_option_match(Some("42")), 42);
        assert_eq!(parse_option_match(Some("x")), 0);
        assert_eq!(parse_option_match(None), 0);
        assert_eq!(parse_option_combinators(Some("42")), 42);
        assert_eq!(parse_option_combinators(Some("x")), 0);
        assert_eq!(parse_option_combinators(None), 0);
    }

    #[test]
    fn divide_ok_and_err() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(1.0, 0.0), Err("division by zero".to_string()));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_unchecked_panics_on_zero() {
        divide_unchecked(1.0, 0.0);
    }

    #[test]
    fn word_count_counts() {
        let wc = count_text("hello world\nfoo bar baz\n");
        assert_eq!(wc.words, 5);
        assert_eq!(wc.lines, 2);
        assert_eq!(wc.chars, 24);
    }

    #[test]
    fn word_count_from_file_works() {
        let f = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(f.path(), "a b c\n").unwrap();
        let wc = word_count_from_file(f.path().to_str().unwrap()).unwrap();
        assert_eq!(wc.words, 3);
    }

    #[test]
    fn word_count_missing_file_errors() {
        assert!(matches!(
            word_count_from_file("/nonexistent/definitely-missing.txt"),
            Err(CliError::Io(_))
        ));
    }
}
