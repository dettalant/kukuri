use crate::core::dialog::Scene;

pub struct Json;

impl Json {
    pub fn export_string(scenes: &Vec<Scene>) -> String {
        serde_json::to_string(scenes).unwrap_or_else(|e| {
            eprintln!("Failed to json serialization: {:?}", e);
            String::new()
        })
    }
}
