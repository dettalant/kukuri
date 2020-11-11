use crate::utils;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use toml;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(default)]
pub struct Config {
    pub use_l10n_output: bool,
    pub separate_output: bool,
    pub orig_locale: String,
    pub default_script_type: String,
    pub outputs: Vec<String>,
    pub l10n_outputs: Vec<String>,
    pub output_dir: PathBuf,
    pub l10n_output_dir: PathBuf,
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
        let mut conf = Config::parse(&s).unwrap_or_else(|e| {
            eprintln!("Toml Parse Error: {:?}", e);
            Config::new()
        });

        conf.output_dir = Self::get_relative_dir(&path, &conf.output_dir);
        conf.l10n_output_dir = Self::get_relative_dir(&path, &conf.l10n_output_dir);

        conf
    }

    fn get_relative_dir<P: AsRef<Path>, P2: AsRef<Path>>(base_path: &P, append_path: &P2) -> PathBuf {
        let mut p = PathBuf::from(base_path.as_ref());
        if !p.is_dir() { p.set_file_name("") };
        p.push(append_path.as_ref());
        p
    }
}


impl Default for Config {
    fn default() -> Config {
        let current_dir = std::env::current_dir().expect("Failed get current directory.");
        Config {
            use_l10n_output: true,
            separate_output: true,
            orig_locale: String::from("en_US"),
            default_script_type: String::from("kukuri"),
            outputs: vec![String::from("gd")],
            l10n_outputs: vec![String::from("po")],
            output_dir: current_dir.clone(),
            l10n_output_dir: current_dir.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::path::PathBuf;

    #[test]
    fn test_set_relative_dir() {
        let tests = [
            ("/tmp", "./test", PathBuf::from("/tmp/test")),
            ("/tmp/", "./test", PathBuf::from("/tmp/test")),
            ("/tmp/kukuri/not_found.file", "./test", PathBuf::from("/tmp/kukuri/test")),
            ("./", "src", PathBuf::from("./src")),
        ];

        for (src0, src1, expected) in &tests {
            assert_eq!(*expected, Config::get_relative_dir(src0, src1));
        }
    }

    #[test]
    fn test_parse() {
        let conf_str0 = "\
use_l10n_output = false
separate_output = false
orig_locale = 'ja_JP'
default_script_type = 'yarn'
outputs = ['gd', 'json']
l10n_outputs = ['po', 'fluent']";

        let conf_str1 = "\
orig_locale = 'fr_FR'
separate_output = false
";

        let conf0 = Config {
            use_l10n_output: false,
            separate_output: false,
            orig_locale: String::from("ja_JP"),
            default_script_type: String::from("yarn"),
            outputs: vec![String::from("gd"), String::from("json")],
            l10n_outputs: vec![String::from("po"), String::from("fluent")],
            ..Default::default()
        };

        let mut conf1 = Config::new();
        conf1.orig_locale = String::from("fr_FR");
        conf1.separate_output = false;

        let tests = [
            (conf_str0, Ok(conf0)),
            (conf_str1, Ok(conf1)),
        ];

        for &(src, ref expected) in &tests {
            assert_eq!(expected, &Config::parse(src));
        }
    }
}
