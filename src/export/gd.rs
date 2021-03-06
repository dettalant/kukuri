use super::json::Json;
use crate::core::dialog::Scenes;

const DEFAULT_GDSCRIPT_TEMPLATE: &'static str = include_str!("../templates/gd");

pub struct GDScript;

impl GDScript {
    fn replace_template<T: AsRef<str>>(json_str: T) -> String {
        DEFAULT_GDSCRIPT_TEMPLATE.replace("$SCENES", json_str.as_ref())
    }

    pub fn export_string(scenes: &Scenes, is_minify: bool) -> String {
        let json_str = Json::export_string(scenes, is_minify);
        Self::replace_template(json_str)
    }
}
