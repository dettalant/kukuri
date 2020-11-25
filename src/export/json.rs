use crate::core::dialog::Scenes;

pub struct Json;

impl Json {
    pub fn export_string(scenes: &Scenes, is_minify: bool) -> String {
        let s = if is_minify {
            serde_json::to_string(scenes)
        } else {
            serde_json::to_string_pretty(scenes)
        };

        s.unwrap_or_else(|e| {
            eprintln!("Failed to json serialization: {:?}", e);
            String::new()
        })
    }
}
