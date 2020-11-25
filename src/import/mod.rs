pub mod kukuri_script;
pub mod kukuri_talkers;

const DEFAULT_FALLBACK_TYPE: ImportType = ImportType::KukuriScript;

pub enum ImportType {
    KukuriScript,
    KukuriTalkers,
    Yarn,
    Ink,
}

impl ImportType {
    pub fn from_typename(typename: &str) -> Self {
        match typename {
            "kukuri" => ImportType::KukuriScript,
            "kukuri_talkers" => ImportType::KukuriTalkers,
            "yarn" => ImportType::Yarn,
            "ink" => ImportType::Ink,
            _ => DEFAULT_FALLBACK_TYPE,
        }
    }

    pub fn from_extension(ext: &str, fallback_str: &str) -> Self {
        let fallback = ImportType::from_typename(fallback_str);
        match ext {
            "kkr" => ImportType::KukuriScript,
            "kkrt" => ImportType::KukuriTalkers,
            "yarn" => ImportType::Yarn,
            "ink" => ImportType::Ink,
            _ => fallback,
        }
    }
}
