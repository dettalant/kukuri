pub mod dialog;

use crate::config::Config;
use crate::export::{json::Json, po::Po, ExportType, L10nExportType};
use crate::import::{kukuri_script::KukuriScript, ImportType};
use crate::utils;
use dialog::{Dialog, DialogBody, DialogKind, Scene};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Kukuri {
    pub conf: Config,
    pub scenes: Vec<Scene>,
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

    pub fn set_output_dir(&mut self, new_dir: PathBuf) {
        self.conf.output_dir = new_dir;
    }

    pub fn set_l10n_output_dir(&mut self, new_dir: PathBuf) {
        self.conf.l10n_output_dir = new_dir;
    }

    fn parse(&mut self, content: &str, ext: &str) {
        let import_type = ImportType::from_extension(ext, &self.conf.default_script_type);

        let mut scenes: Vec<Scene> = match import_type {
            ImportType::Yarn => {
                println!("Sorry, YarnSpinner script is not supported currently.");
                Vec::new()
            }
            ImportType::Ink => {
                println!("Sorry, Ink script is not supported currently.");
                Vec::new()
            }
            ImportType::KukuriScript => KukuriScript::parse(content),
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
            Err(e) => eprintln!("Failed to load {}: {:?}", path.as_ref().display(), e),
        };
    }

    pub fn export(&mut self) {
        if self.scenes.is_empty() {
            return;
        };

        if self.conf.use_l10n_output {
            // Change serialize setting.
            // if this var enable, ChoiceData.label has not serialized.
            std::env::set_var("KUKURI_IS_EXCLUDE_ORIG_TEXT", "TRUE");

            Self::exclude_orig_text(&mut self.scenes);
        }

        let mut exports: Vec<ExportType> = self
            .conf
            .outputs
            .iter()
            .map(|s| ExportType::parse(s))
            .collect();

        exports.dedup();

        // export type
        for et in exports {
            let s = match et {
                ExportType::Json => Json::export_string(&self.scenes),
                // placeholder
                _ => String::new(),
            };

            // TODO: multiple output feature
            let mut path = self.conf.output_dir.clone();
            path.push(format!("{}{}", "output", et.extension()));

            utils::write_file(path.as_ref(), &s).expect("Unable to write file.");
        }
    }

    pub fn l10n_export(&self) {
        if self.scenes.is_empty() || !self.conf.use_l10n_output {
            return;
        };

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
                L10nExportType::Po => Po::export_string(&self.scenes, locale),
            };

            let mut path = self.conf.l10n_output_dir.clone();
            path.push(format!("{}{}", locale, et.extension()));

            // TODO: make export directory feature
            utils::write_file(path.as_ref(), &s).expect("Unable to write file.");
        }
    }

    fn exclude_orig_text(scenes: &mut Vec<Scene>) {
        fn dialog_process(dialog: &mut Dialog) {
            match dialog.kind {
                DialogKind::Dialog => dialog.args.clear(),
                DialogKind::Choices => dialog
                    .args
                    .iter_mut()
                    .for_each(|body| choices_process(body)),
                _ => {}
            }
        }

        fn choices_process(body: &mut DialogBody) {
            if let DialogBody::Choice(ref mut cd) = body {
                for dialog in &mut cd.dialogs {
                    dialog_process(dialog);
                }
            }
        }

        for scene in scenes {
            for dialog in &mut scene.dialogs {
                dialog_process(dialog)
            }
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
    use crate::core::dialog::{ChoiceData, Dialog, DialogBody, DialogKind, Scene};
    use std::env;

    #[test]
    fn test_set_output_dir() {
        let current_dir = env::current_dir().expect("Failed get current dir.");
        let tmp_dir = env::temp_dir();

        let kkr0 = Kukuri::from_config(Config::new());
        let mut kkr1 = Kukuri::from_config(Config::new());
        kkr1.set_output_dir(tmp_dir.clone());

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
        kkr1.set_l10n_output_dir(tmp_dir.clone());

        let tests = [(current_dir, kkr0), (tmp_dir, kkr1)];

        for &(ref src, ref expected) in &tests {
            assert_eq!(*src, *expected.conf.l10n_output_dir);
        }
    }

    #[test]
    fn test_exclude_orig_text() {
        let (dkd, dkc) = (DialogKind::Dialog, DialogKind::Choices);
        let mut kkr = Kukuri::new();
        let mut cd = ChoiceData::new();
        cd.dialogs = vec![
            Dialog::from_dialog_data(
                dkd,
                "TestInnerDialog0",
                vec![DialogBody::gen_text("いんなーだいあろぐてすと0")],
            ),
            Dialog::from_dialog_data(
                dkd,
                "TestInnerDialog1",
                vec![DialogBody::gen_text("いんなーだいあろぐてすと1")],
            ),
        ];

        let mut scene = Scene::new();
        scene.dialogs = vec![
            Dialog::from_dialog_data(
                dkd,
                "TestDialog0",
                vec![DialogBody::gen_text("だいあろぐてすと0")],
            ),
            Dialog::from_dialog_data(
                dkd,
                "TestDialog1",
                vec![DialogBody::gen_text("だいあろぐてすと1")],
            ),
            Dialog::from_dialog_data(dkc, "TestChoices", vec![DialogBody::Choice(cd)]),
        ];
        kkr.scenes = vec![scene];

        let mut expected_scene = Scene::new();
        let mut expected_cd = ChoiceData::new();
        expected_cd.dialogs = vec![
            Dialog::from_dialog_data(dkd, "TestInnerDialog0", Vec::new()),
            Dialog::from_dialog_data(dkd, "TestInnerDialog1", Vec::new()),
        ];

        expected_scene.dialogs = vec![
            Dialog::from_dialog_data(dkd, "TestDialog0", Vec::new()),
            Dialog::from_dialog_data(dkd, "TestDialog1", Vec::new()),
            Dialog::from_dialog_data(dkc, "TestChoices", vec![DialogBody::Choice(expected_cd)]),
        ];

        Kukuri::exclude_orig_text(&mut kkr.scenes);

        assert_eq!(vec![expected_scene], kkr.scenes);
    }
}
