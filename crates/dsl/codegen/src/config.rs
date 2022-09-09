use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Write},
    fs::read_to_string,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, IteratorExt, Result, ResultExt};
use indexmap::IndexMap;

#[derive(Debug)]
pub(crate) enum ConfigError {
    NotCargo,
    Io,
    Toml,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotCargo => f.write_str("`codegen` can only be executed with `cargo run`"),
            ConfigError::Io => f.write_str("during reading an io error occurred"),
            ConfigError::Toml => f.write_str("`types.toml` contains invalid toml"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Is {
    Literal,
    Trivia,
    InfixOp,
    PrefixOp,
    PostfixOp,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Kind {
    pub(crate) regex: Option<Vec<String>>,
    pub(crate) token: Option<String>,

    #[serde(default)]
    pub(crate) is: HashSet<Is>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Syntax {}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) kind: IndexMap<String, Kind>,
    syntax: IndexMap<String, Syntax>,
}

impl Config {
    pub(crate) fn load() -> Result<Self, ConfigError> {
        let path = path().ok_or(ConfigError::NotCargo).into_report()?;

        let contents = read_to_string(path)
            .into_report()
            .change_context(ConfigError::Io)?;

        let this = toml::from_str(&contents)
            .into_report()
            .change_context(ConfigError::Toml)?;

        Ok(this)
    }
}

pub(crate) fn path() -> Option<PathBuf> {
    let env = option_env!("CARGO_MANIFEST_DIR")?;

    Some(Path::new(env).join("types.toml"))
}
