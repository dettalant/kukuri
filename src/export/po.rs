use crate::core::dialog::{Scene, DialogKind, DialogBody};

const PO_BEGINNING_LINES: &str = "\
msgid \"\"
msgstr \"\"
\"Last-Translator: Automatically generated\\n\"
\"Language-Team: none\\n\"
\"MIME-Version: 1.0\\n\"
\"Content-Type: text/plain; charset=UTF-8\\n\"
\"Plural-Forms: nplurals=1; plural=0;\\n\"";

pub struct Po;

impl Po {
    pub fn export_string(scenes: &Vec<Scene>, locale: &str) -> String {
        // output string
        let mut out_s = Self::gen_init_string(locale);

        for scene in scenes {
            // nest level
            out_s.push_str(&Self::parse_scene(scene));
        }

        out_s
    }

    pub fn parse_scene(scene: &Scene) -> String {
        let mut out_s = String::new();

        for dialog in &scene.dialogs {
            if dialog.kind == DialogKind::Command {continue};

            match dialog.kind {
                DialogKind::Dialog => {
                    let DialogBody::Text(dialog_body) = &dialog.args[0];
                    out_s.push_str(&Self::gen_msgid(&dialog.id));
                    out_s.push_str(&Self::gen_msgstr(dialog_body));
                },
                _ => {},
            }

        }

        out_s
    }


    pub fn gen_init_string(locale: &str) -> String {
        format!("{}\n\"Language: {}\\n\"\n\n", PO_BEGINNING_LINES, locale)
    }

    pub fn gen_msgid(s: &str) -> String {
        format!("msgid \"{}\"\n", Self::quote_escape(s))
    }

    pub fn gen_msgstr(s: &str) -> String {
        // add double line break
        format!("msgstr \"{}\"\n\n", Self::quote_escape(s))
    }

    fn quote_escape(s: &str) -> String {
        s.replace('"', "\\\"")
    }
}

#[cfg(test)]
mod tests {
    use super::Po;

    #[test]
    fn test_gen_init_string() {
        let expected = "\
msgid \"\"
msgstr \"\"
\"Last-Translator: Automatically generated\\n\"
\"Language-Team: none\\n\"
\"MIME-Version: 1.0\\n\"
\"Content-Type: text/plain; charset=UTF-8\\n\"
\"Plural-Forms: nplurals=1; plural=0;\\n\"
\"Language: ja\\n\"

";
        assert_eq!(expected, Po::gen_init_string("ja"));
    }

}
