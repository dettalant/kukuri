pub mod gd;
pub mod json;
pub mod po;

const DEFAULT_EXPORT_TYPE: ExportType = ExportType::GDScript;
const DEFAULT_L10N_EXPORT_TYPE: L10nExportType = L10nExportType::Po;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExportType {
    GDScript,
    Json,
}

impl ExportType {
    pub fn parse(s: &str) -> Self {
        match s {
            "gd" => ExportType::GDScript,
            "json" => ExportType::Json,
            _ => DEFAULT_EXPORT_TYPE,
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ExportType::GDScript => "gd",
            ExportType::Json => "json",
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
        match self {
            L10nExportType::Po => "po",
        }
    }
}
