use crate::core::dialog::{ChoiceData, Dialog, DialogBody, DialogKind, Scene};
use crate::core::talker::Talker;

const DEFAULT_PO_TEMPLATE: &'static str = include_str!("../templates/po");

pub struct Po;

impl Po {
    pub fn export_string(scenes: &Vec<Scene>, talkers: &Vec<Talker>, locale: &str) -> String {
        // output string
        let mut out_s = Self::gen_init_string(locale);

        for talker in talkers {
            out_s.push_str(&Self::convert_talker(talker));
        }

        for scene in scenes {
            out_s.push_str(&Self::convert_scene(scene));
        }

        out_s
    }

    fn convert_scene(scene: &Scene) -> String {
        let mut out_s = String::new();

        for dialog in &scene.dialogs {
            out_s.push_str(&Self::convert_dialog(dialog));
        }

        out_s
    }

    fn convert_dialog(dialog: &Dialog) -> String {
        if dialog.kind == DialogKind::Command {
            return String::new();
        };

        let mut out_s = String::new();

        match dialog.kind {
            DialogKind::Dialog => {
                if let DialogBody::Text(dialog_body) = &dialog.args[0] {
                    out_s.push_str(&Self::gen_msgid(&dialog.id));
                    out_s.push_str(&Self::gen_msgstr(dialog_body));
                };
            }
            DialogKind::Choices => {
                for choice in &dialog.args {
                    if let DialogBody::Choice(cd) = choice {
                        out_s.push_str(&Self::convert_choice(cd));
                    }
                }
            }
            _ => {}
        }

        out_s
    }

    fn convert_choice(cd: &ChoiceData) -> String {
        let mut out_s = String::new();
        out_s.push_str(&Self::gen_msgid(&cd.id));
        out_s.push_str(&Self::gen_msgstr(&cd.label));
        for dialog in &cd.dialogs {
            out_s.push_str(&Self::convert_dialog(dialog));
        }

        out_s
    }

    fn convert_talker(talker: &Talker) -> String {
        let mut out_s = Self::gen_msgid(talker.id());
        out_s.push_str(&Self::gen_msgstr(talker.name()));
        out_s
    }

    fn gen_init_string(locale: &str) -> String {
        let mut s = DEFAULT_PO_TEMPLATE.replace("$LOCALE", locale);
        s.push_str("\n\n");
        s
    }

    fn gen_msgid<T: AsRef<str>>(s: T) -> String {
        format!("msgid \"{}\"\n", Self::quote_escape(s.as_ref()))
    }

    fn gen_msgstr<T: AsRef<str>>(s: T) -> String {
        // add double line break
        format!("msgstr \"{}\"\n\n\n", Self::quote_escape(s.as_ref()))
    }

    fn quote_escape(s: &str) -> String {
        s.replace('"', "\\\"")
    }
}

#[cfg(test)]
mod tests {
    use super::Po;
    use crate::core::dialog::{ChoiceData, Dialog, DialogBody, DialogKind};
    use crate::core::talker::Talker;

    #[test]
    fn test_gen_init_string() {
        let expected = "\
msgid \"\"
msgstr \"\"
\"Last-Translator: Automatically generated\\n\"
\"Language-Team: none\\n\"
\"Language: ja\\n\"
\"MIME-Version: 1.0\\n\"
\"Content-Type: text/plain; charset=UTF-8\\n\"
\"Plural-Forms: nplurals=1; plural=0;\\n\"


";
        assert_eq!(expected, Po::gen_init_string("ja"));
    }

    #[test]
    fn test_convert_dialog() {
        let d0 = Dialog::from_dialog_data(
            DialogKind::Dialog,
            "TestDialog0",
            vec![DialogBody::gen_text("てててすと")],
        );
        let expected0 = "\
msgid \"TestDialog0\"
msgstr \"てててすと\"


";
        assert_eq!(expected0, Po::convert_dialog(&d0));

        let mut cd0 = ChoiceData::from_texts("TestChoice0", "てすと選択肢0");
        cd0.dialogs.push(Dialog::from_dialog_data(
            DialogKind::Dialog,
            "TestInnerDialog0",
            vec![DialogBody::gen_text("てすとだいあろぐ")],
        ));

        let cd1 = ChoiceData::from_texts("TestChoice1", "てすと選択肢1");

        let d1 = Dialog::from_dialog_data(
            DialogKind::Choices,
            "",
            vec![DialogBody::Choice(cd0), DialogBody::Choice(cd1)],
        );

        let expected1 = "\
msgid \"TestChoice0\"
msgstr \"てすと選択肢0\"


msgid \"TestInnerDialog0\"
msgstr \"てすとだいあろぐ\"


msgid \"TestChoice1\"
msgstr \"てすと選択肢1\"


";

        assert_eq!(expected1, Po::convert_dialog(&d1));
    }

    #[test]
    fn test_convert_talker() {
        let expected = "\
msgid \"A\"
msgstr \"Alpha\"


";
        let talker = Talker::from_strs("A", "Alpha");
        assert_eq!(expected, Po::convert_talker(&talker));
    }
}
