use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    fs::read_to_string,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, Result, ResultExt};
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
            Self::NotCargo => f.write_str("`codegen` can only be executed with `cargo run`"),
            Self::Io => f.write_str("during reading an io error occurred"),
            Self::Toml => f.write_str("`types.toml` contains invalid toml"),
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

    pub(crate) shortcut: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Syntax {
    pub(crate) shortcut: Option<String>,
    pub(crate) contextual: Option<String>,
    pub(crate) composite: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) kind: IndexMap<String, Kind>,
    pub(crate) syntax: IndexMap<String, Syntax>,
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
