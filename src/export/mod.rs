pub mod po;

const DEFAULT_EXPORT_TYPE: ExportType = ExportType::GDScript;
const DEFAULT_L10N_EXPORT_TYPE: L10nExportType = L10nExportType::Po;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExportType {
    GDScript,
    // Json,
}

impl ExportType {
    pub fn parse(s: &str) -> Self {
        match s {
            "gd" => ExportType::GDScript,
            _ => DEFAULT_EXPORT_TYPE,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum L10nExportType {
    Po,
    // Csv,
    // Fluent,
}


impl L10nExportType {
    pub fn parse(s: &str) -> Self {
        match s {
            "po" => L10nExportType::Po,
            _ => DEFAULT_L10N_EXPORT_TYPE,
        }
    }

    pub fn extension(&self) -> &str {
        let ext = match self {
            L10nExportType::Po => ".po"
        };

        ext
    }
}
