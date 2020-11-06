use super::utils;
use std::path::{Path};
use serde::{Serialize, Deserialize};
use toml;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub use_l10n_output: bool,
    pub separate_output: bool,
    pub orig_locale: String,
    pub default_script_type: String,
    pub outputs: Vec<String>,
    pub l10n_outputs: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ..Default::default()
        }
    }

    pub fn parse(s: &str) -> Result<Self, toml::de::Error> {
        let config: Config = toml::from_str(s)?;
        Ok(config)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let s = utils::read_file(path.as_ref())
            .unwrap_or_else(|_| {
                eprintln!("Failed to load: {}", path.as_ref().display());
                String::new()
            });
        Config::parse(&s).unwrap_or_else(|e| {
            eprintln!("Toml Parse Error: {:?}", e);
            Config::new()
        })
    }
}


impl Default for Config {
    fn default() -> Config {
        Config {
            use_l10n_output: true,
            separate_output: true,
            orig_locale: String::from("en_US"),
            default_script_type: String::from("kukuri"),
            outputs: vec![String::from("gd")],
            l10n_outputs: vec![String::from("po")],
        }
    }
}


