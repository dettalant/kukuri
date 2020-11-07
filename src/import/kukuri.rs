use crate::core::dialog::{Dialog, DialogBody, DialogKind, Scene};

// TODO: toml + serde implement
pub struct KukuriScript;

impl KukuriScript {
    pub fn parse(content: &str) -> Vec<Scene> {
        let mut scenes: Vec<Scene> = Vec::new();
        let mut is_header = false;
        let mut scene_title = String::new();
        let mut scene_dialogs: Vec<Dialog> = Vec::new();

        for full_line in content.lines() {
            let line = KukuriScript::trim_comment(full_line.trim());
            // Empty line is skip.
            if line.is_empty() { continue };

            if is_header {
                // Header symbol check
                if KukuriScript::is_header_symbol(line) {
                    is_header = false;
                    continue;
                }

                let (header_id, header_body) = KukuriScript::header_split(line);

                if header_id == "title" {
                    let title_chars: Vec<char> = header_body.chars().collect();
                    if title_chars[0] == '\'' || title_chars[0] == '"' {
                        let max = title_chars.len() - 1;
                        let title: String = title_chars.iter()
                            .enumerate()
                            .filter(|&(i, _)| i > 0 && i < max)
                            .map(|(_, c)| c)
                            .collect();
                        scene_title = title;
                    }
                }

                continue;
            }

            if KukuriScript::is_header_symbol(line) {
                is_header = true;
                continue;
            }

            if KukuriScript::is_scene_end_symbol(line) {
                scenes.push(Scene::from_scene_data(
                    scene_title.clone(),
                    scene_dialogs.clone()
                ));

                scene_title.clear();
                scene_dialogs.clear();
            }

            let kind = KukuriScript::get_dialog_kind(line);
            let (id, args) = match kind {
                DialogKind::Dialog => KukuriScript::get_dialog_data(line),
                DialogKind::Command => KukuriScript::get_command_data(line),
                DialogKind::Choices => {
                    let id = String::from("choices");
                    let body = vec![DialogBody::Text(String::from("test"))];

                    (id, body)
                },
            };

            scene_dialogs.push(Dialog::from_dialog_data(kind, id, args));
        }

        // temporary return
        scenes
    }

    fn is_header_symbol(line: &str) -> bool {
        let mut is_header_symbol = true;
        let mut chars = line.chars();
        for _ in 0..3 {
            match chars.next() {
                Some(c) if c == '+' => {},
                _ => {
                    is_header_symbol = false;
                    break;
                }
            }
        }
        is_header_symbol
    }

    fn is_scene_end_symbol(line: &str) -> bool {
        let mut is_header_symbol = true;
        let mut chars = line.chars();
        for _ in 0..3 {
            match chars.next() {
                Some(c) if c == '=' => {},
                _ => {
                    is_header_symbol = false;
                    break;
                }
            }
        }
        is_header_symbol
    }

    fn header_split(line: &str) -> (&str, &str) {
        let mut iter = line.splitn(2, '=');

        let id = iter.next().unwrap_or("");
        let body = iter.next().unwrap_or("");
        (id, body)
    }

    fn trim_comment(line: &str) -> &str {
        let mut bs_i = 0;
        let symbol_idx = line.char_indices().find(|&(i, c)| {
            if c == '\\' {
                bs_i = i;
            }

            let mut is_comment = c == '#';

            if bs_i + 1 == i {
                is_comment = false;
            }

            is_comment
        });

        match symbol_idx {
            Some((i, _)) => line.split_at(i).0,
            None => line,
        }
    }

    fn get_dialog_data(line: &str) -> (String, Vec<DialogBody>) {
        let mut iter = line.splitn(2, ':');

        let id = String::from(iter.next().unwrap_or("").trim());
        let body = String::from(iter.next().unwrap_or("").trim());

        if !body.is_empty() {
            (id, vec![DialogBody::Text(body)])
        } else {
            (String::new(), vec![DialogBody::Text(id)])
        }
    }

    fn get_command_data(line: &str) -> (String, Vec<DialogBody>) {
        let cmd_str = line.splitn(2, '$')
            .nth(1)
            .unwrap_or("")
            .trim();

        let mut iter = cmd_str.split_whitespace();

        let id = String::from(iter.nth(0).unwrap_or(""));
        let args: Vec<DialogBody> = iter.map(|s| {
            let body = String::from(s);
            DialogBody::Text(body)
         }).collect();

        (id, args)
    }

    fn get_dialog_kind(full_line: &str) -> DialogKind {
        let line = full_line.trim();
        // '\0' is placeholder char
        let c = line.chars().nth(0).unwrap_or('\0');

        match c {
            '$' => DialogKind::Command,
            '*' | '+' | '-' => DialogKind::Choices,
            _ => DialogKind::Dialog,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KukuriScript;
    use crate::core::dialog::{DialogBody, DialogKind};

    #[test]
    fn test_get_dialog_data() {
        let tests = [
            ("A: test dialog", (String::from("A"), vec![DialogBody::Text(String::from("test dialog"))])),
            ("non-talker dialog", (String::new(), vec![DialogBody::Text(String::from("non-talker dialog"))]))
        ];

        for &(src, ref expected) in &tests {
            let a = &KukuriScript::get_dialog_data(src);
            assert_eq!(expected.0, a.0);
            assert_eq!(expected.1, a.1);
        }
    }

    #[test]
    fn test_get_dialog_kind() {
        let tests = [
            ("$ jump ttt", DialogKind::Command),
            ("A: test dialog", DialogKind::Dialog),
            ("non-talker dialog", DialogKind::Dialog),
            ("  # commented line", DialogKind::Dialog),
            ("* choice1", DialogKind::Choices),
            ("+ choice2", DialogKind::Choices),
            ("- choice3", DialogKind::Choices),
        ];

        for &(src, ref expected) in &tests {
            assert_eq!(expected, &KukuriScript::get_dialog_kind(src))
        }
    }

    #[test]
    fn test_get_command_data() {
        // new dialog body vector
        let new_db_v = |x: Vec<&str>| {
            let mut v: Vec<DialogBody> = Vec::new();
            for arg in x {
                v.push(DialogBody::Text(String::from(arg)));
            }
            v
        };

        let srcs = [
            "$ jump TestDialog",
            "$ test ttt ttt ttt ttt ttt ttt ttt ttt",
            "$ set VAR 1",
        ];

        let expected = [
            ("jump", vec!["TestDialog"]),
            ("test", vec!["ttt", "ttt", "ttt", "ttt", "ttt", "ttt", "ttt", "ttt"]),
            ("set", vec!["VAR", "1"]),
        ];

        let mut tests = Vec::new();
        for i in 0..srcs.len() {
            let expected0 = String::from(expected[i].0);
            let expected1 = new_db_v(expected[i].1.clone());
            tests.push((srcs[i], (expected0, expected1)));
        }

        for (src, (e0, e1)) in tests {
            let (a0, a1) = KukuriScript::get_command_data(src);
            assert_eq!(e0, a0);
            assert_eq!(e1, a1);
        }
    }

    #[test]
    fn test_trim_comment() {
        let tests = [
            ("# head of line comment", ""),
            ("end of line comment# comment", "end of line comment"),
            ("middle of # line comment", "middle of "),
            ("A: comment nothing", "A: comment nothing"),
            ("comment nothing", "comment nothing"),
            ("# $ cmd comment test", ""),
            ("$ cmd comment # test", "$ cmd comment "),
        ];

        for &(src, ref expected) in &tests {
            assert_eq!(expected, &KukuriScript::trim_comment(src));
        }
    }

    #[test]
    fn test_is_scene_end_symbol() {
        let tests = [
            ("===", true),
            ("==not scene end==", false),
            ("========", true),
            ("   === untrim line", false),
            ("not===scene end", false),
        ];

        for &(src, ref expected) in &tests {
            assert_eq!(expected, &KukuriScript::is_scene_end_symbol(src));
        }
    }

    #[test]
    fn test_is_header_symbol() {
        let tests = [
            ("+++", true),
            ("++++++", true),
            ("++not header symbol++", false),
            ("   +++ untrim line", false),
            ("not+++header symbol", false),
        ];

        for &(src, ref expected) in &tests {
            assert_eq!(expected, &KukuriScript::is_header_symbol(src));
        }
    }
}
