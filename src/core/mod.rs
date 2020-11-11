pub mod dialog;

use std::path::{Path, PathBuf};
use dialog::Scene;
use crate::config::Config;
use crate::utils;
use crate::import::{ImportType, kukuri::KukuriScript};
use crate::export::{L10nExportType, po::Po};

#[derive(Debug)]
pub struct Kukuri {
    pub conf: Config,
    pub scenes: Vec<Scene>,
}

impl Kukuri {
    // pub fn new() -> Self {
    //     Kukuri {
    //         ..Default::default()
    //     }
    // }

    pub fn from_config(conf: Config) -> Self {
        Kukuri {
            conf,
            ..Default::default()
        }
    }

    pub fn set_output_dir(&mut self, new_dir: PathBuf) {
        self.conf.output_dir = new_dir;
    }


    pub fn set_l10n_output_dir(&mut self, new_dir: PathBuf) {
        self.conf.l10n_output_dir = new_dir;
    }


    fn parse(&mut self, content: &str, ext: &str) {
        let import_type = ImportType::from_extension(
            ext,
            &self.conf.default_script_type
        );

        let mut scenes: Vec<Scene> = match import_type {
            ImportType::Yarn => {
                println!("Sorry, YarnSpinner script is not supported currently.");
                Vec::new()
            },
            ImportType::Ink => {
                println!("Sorry, Ink script is not supported currently.");
                Vec::new()
            },
            ImportType::KukuriScript => KukuriScript::parse(content)
        };

        self.scenes.append(&mut scenes);
    }

    pub fn import<P: AsRef<Path>>(&mut self, path: P) {
        let ext = match path.as_ref().extension() {
            Some(s) => s.to_str().unwrap_or(""),
            None => "",
        };

        match utils::read_file(path.as_ref()) {
            Ok(s) => self.parse(&s, ext),
            Err(e) => eprintln!("Failed to load {}: {:?}", path.as_ref().display(), e)
        };
    }

    // pub fn export(self) {
    //     if self.scenes.is_empty() { return };
    //
    //     for t in self.conf.outputs {
    //
    //     }
    // }

    pub fn l10n_export(self) {
        if self.scenes.is_empty() || !self.conf.use_l10n_output { return };

        let mut exports: Vec<L10nExportType> = self.conf.l10n_outputs.iter()
            .map(|s| L10nExportType::parse(s))
            .collect();

        exports.dedup();

        // export type
        for et in exports {
            let locale = (self.conf.orig_locale.as_str()).splitn(2, "_")
                .nth(0)
                .unwrap_or("en");
            let ext = et.extension();
            let mut path = self.conf.l10n_output_dir.clone();
            path.push(
                format!("{}{}", locale, ext)
            );

            let s = match et {
                L10nExportType::Po => Po::export_string(&self.scenes, locale),
            };

            // TODO: create export directory feature
            utils::write_file(path.as_ref(), &s).expect("Unable to write file.");
        }

    }
}

impl Default for Kukuri {
    fn default() -> Self {
        Kukuri {
            conf: Config::new(),
            scenes: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Kukuri;
    use crate::config::Config;
    use std::env;

    #[test]
    fn test_set_output_dir() {
        let current_dir = env::current_dir().expect("Failed get current dir.");
        let tmp_dir = env::temp_dir();

        let kkr0 = Kukuri::from_config(Config::new());
        let mut kkr1 = Kukuri::from_config(Config::new());
        kkr1.set_output_dir(tmp_dir.clone());

        let tests = [
            (current_dir, kkr0),
            (tmp_dir, kkr1),
        ];

        for (ref src, ref expected) in &tests {
            assert_eq!(*src, *expected.conf.output_dir);
        }
    }

    #[test]
    fn test_set_l10n_output_dir() {
        let current_dir = env::current_dir().expect("Failed get current dir.");
        let tmp_dir = env::temp_dir();

        let kkr0 = Kukuri::from_config(Config::new());
        let mut kkr1 = Kukuri::from_config(Config::new());
        kkr1.set_l10n_output_dir(tmp_dir.clone());

        let tests = [
            (current_dir, kkr0),
            (tmp_dir, kkr1),
        ];

        for &(ref src, ref expected) in &tests {
            assert_eq!(*src, *expected.conf.l10n_output_dir);
        }
    }
}
