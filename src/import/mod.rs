pub mod kukuri_script;

const DEFAULT_FALLBACK_TYPE: ImportType = ImportType::KukuriScript;

pub enum ImportType {
    KukuriScript,
    Yarn,
    Ink,
}

impl ImportType {
    pub fn from_typename(typename: &str) -> Self {
        match typename {
            "kukuri" => ImportType::KukuriScript,
            "yarn" => ImportType::Yarn,
            "ink" => ImportType::Ink,
            _ => DEFAULT_FALLBACK_TYPE,
        }
    }

    pub fn from_extension(ext: &str, fallback_str: &str) -> Self {
        let fallback = ImportType::from_typename(fallback_str);
        match ext {
            "kkr" => ImportType::KukuriScript,
            "yarn" => ImportType::Yarn,
            "ink" => ImportType::Ink,
            _ => fallback,
        }
    }
}
