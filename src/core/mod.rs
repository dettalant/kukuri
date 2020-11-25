pub mod dialog;
pub mod kukuri_data;
pub mod talker;

use crate::config::Config;
use crate::export::{gd::GDScript, json::Json, po::Po, ExportType, L10nExportType};
use crate::import::{kukuri_script::KukuriScript, kukuri_talkers::KukuriTalkers, ImportType};
use crate::utils;
use dialog::{Scene, Scenes};
use kukuri_data::KukuriData;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use talker::Talker;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Kukuri {
    pub conf: Config,
    pub inputs: Vec<PathBuf>,
}

impl Default for Kukuri {
    fn default() -> Self {
        Kukuri {
            conf: Config::new(),
            inputs: Vec::new(),
        }
    }
}

impl Kukuri {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Kukuri {
            ..Default::default()
        }
    }

    pub fn from_config(conf: Config) -> Self {
        Kukuri {
            conf,
            ..Default::default()
        }
    }

    pub fn set_output_dir<T: AsRef<str>>(&mut self, new_dir: T) {
        self.conf.output_dir = PathBuf::from(new_dir.as_ref());
    }

    pub fn set_l10n_output_dir<T: AsRef<str>>(&mut self, new_dir: T) {
        self.conf.l10n_output_dir = PathBuf::from(new_dir.as_ref());
    }

    pub fn append_input<T: AsRef<str>>(&mut self, path: T) {
        self.inputs.push(PathBuf::from(path.as_ref()));
    }

    pub fn run(&self) {
        if self.conf.separate_output {
            self.run_with_separate_output();
            return;
        }

        let mut scenes = Vec::new();
        let mut talkers = Vec::new();

        self.inputs
            .clone()
            .iter()
            .for_each(|p| match self.import(p) {
                KukuriData::Scenes(mut sc) => scenes.append(&mut sc),
                KukuriData::Talkers(mut tk) => talkers.append(&mut tk),
            });

        if self.conf.use_l10n_output {
            self.l10n_export(&scenes, &talkers);
        }
        let shm = Kukuri::scenes_to_hashmap(&scenes);
        self.export(&shm, "output");
    }

    fn run_with_separate_output(&self) {
        fn fallback_filestem(i: usize) -> String {
            format!("{}{}", "output", i)
        }

        let mut exported_scenes = Vec::new();
        let mut talkers = Vec::new();

        for (i, p) in self.inputs.clone().iter().enumerate() {
            let mut scenes = Vec::new();
            match self.import(p) {
                KukuriData::Scenes(mut sc) => scenes.append(&mut sc),
                KukuriData::Talkers(mut tk) => talkers.append(&mut tk),
            };

            let file_stem = match p.file_stem() {
                Some(s) => s
                    .to_os_string()
                    .into_string()
                    .unwrap_or(fallback_filestem(i)),
                None => fallback_filestem(i),
            };
            let shm = Kukuri::scenes_to_hashmap(&scenes);

            self.export(&shm, file_stem);
            exported_scenes.append(&mut scenes);
        }

        // l10n_export in a lump
        if self.conf.use_l10n_output {
            self.l10n_export(&exported_scenes, &talkers);
        }
    }

    fn parse(&self, content: &str, ext: &str) -> KukuriData {
        let import_type = ImportType::from_extension(ext, &self.conf.default_script_type);

        match import_type {
            ImportType::Yarn => {
                println!("Sorry, YarnSpinner script is not supported currently.");
                KukuriData::new()
            }
            ImportType::Ink => {
                println!("Sorry, Ink script is not supported currently.");
                KukuriData::new()
            }
            ImportType::KukuriScript => KukuriScript::parse(content),
            ImportType::KukuriTalkers => KukuriTalkers::parse(content),
        }
    }

    fn import<P: AsRef<Path>>(&self, path: P) -> KukuriData {
        let ext = match path.as_ref().extension() {
            Some(s) => s.to_str().unwrap_or(""),
            None => "",
        };

        match utils::read_file(path.as_ref()) {
            Ok(s) => self.parse(&s, ext),
            Err(e) => {
                eprintln!("Failed to load {}: {:?}", path.as_ref().display(), e);
                KukuriData::new()
            }
        }
    }

    fn export<T: AsRef<str>>(&self, scenes: &Scenes, file_stem: T) {
        if scenes.is_empty() {
            return;
        };

        if self.conf.use_l10n_output {
            // Change serialize setting.
            // if this var enable, ChoiceData.label has not serialized.
            std::env::set_var("KUKURI_IS_EXCLUDE_ORIG_TEXT", "TRUE");
        }

        let output_dir = &self.conf.output_dir;
        if !output_dir.exists() {
            if let Err(e) = utils::mkdir_recursive(output_dir) {
                eprintln!("Kukuri::export() make export dir error: {:?}", e);
                println!("export skipped");
                return;
            }
        }

        let mut exports: Vec<ExportType> = self
            .conf
            .outputs
            .iter()
            .map(|s| ExportType::parse(s))
            .collect();

        exports.dedup();

        let is_minify = self.conf.minified_output;

        // export type
        for et in exports {
            let s = match et {
                ExportType::Json => Json::export_string(&scenes, is_minify),
                ExportType::GDScript => GDScript::export_string(&scenes, is_minify),
            };

            // TODO: multiple output feature
            let mut path = output_dir.clone();
            path.push(format!("{}.{}", file_stem.as_ref(), et.extension()));

            utils::write_file(path, &s).expect("Unable to write file.");
        }
    }

    fn l10n_export<T: AsRef<Vec<Scene>>, T2: AsRef<Vec<Talker>>>(&self, scenes: T, talkers: T2) {
        let scenes = scenes.as_ref();
        let talkers = talkers.as_ref();
        if (scenes.is_empty() && talkers.is_empty()) || !self.conf.use_l10n_output {
            return;
        };

        let output_dir = &self.conf.l10n_output_dir;
        if !output_dir.exists() {
            if let Err(e) = utils::mkdir_recursive(output_dir) {
                eprintln!("Kukuri::l10n_export() make export dir error: {:?}", e);
                println!("l10n export skipped");
                return;
            }
        }

        let mut exports: Vec<L10nExportType> = self
            .conf
            .l10n_outputs
            .iter()
            .map(|s| L10nExportType::parse(s))
            .collect();

        exports.dedup();

        // export type
        for et in exports {
            let locale = (self.conf.orig_locale.as_str())
                .splitn(2, "_")
                .nth(0)
                .unwrap_or("en");

            let s = match et {
                L10nExportType::Po => Po::export_string(scenes, talkers, locale),
            };

            let mut path = output_dir.clone();
            path.push(format!("{}.{}", locale, et.extension()));

            // TODO: make export directory feature
            utils::write_file(path, &s).expect("Unable to write file.");
        }
    }

    fn scenes_to_hashmap(scenes: &Vec<Scene>) -> Scenes {
        // scenes hash map
        let mut shm = HashMap::new();
        for sc in scenes {
            shm.insert(sc.title.clone(), sc.dialogs.clone());
        }
        shm
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
        kkr1.set_output_dir(tmp_dir.to_str().expect("Failed tmpdir to_str()"));

        let tests = [(current_dir, kkr0), (tmp_dir, kkr1)];

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
        kkr1.set_l10n_output_dir(tmp_dir.to_str().expect("Failed tmpdir to_str()"));

        let tests = [(current_dir, kkr0), (tmp_dir, kkr1)];

        for &(ref src, ref expected) in &tests {
            assert_eq!(*src, *expected.conf.l10n_output_dir);
        }
    }
}
