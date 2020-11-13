use crate::core::dialog::{ChoiceData, Dialog, DialogBody, DialogKind, Scene};
use serde::{Deserialize, Serialize};

pub struct KukuriScript;

impl KukuriScript {
    pub fn parse(content: &str) -> Vec<Scene> {
        let mut sp_data = SceneProcessData::new();
        let mut scenes: Vec<Scene> = Vec::new();
        // current scene
        let mut sc = Scene::new();

        for full_line in content.lines() {
            sp_data.line_count_up();
            let line = Self::trim_comment(full_line);

            if line.trim().is_empty() {
                continue;
            };

            if sp_data.is_header {
                Self::header_process(line, &mut sp_data, &mut sc);
                continue;
            }

            if Self::is_header_symbol(line) {
                sp_data.is_header = true;
                continue;
            }

            if Self::is_scene_end_symbol(line) {
                // scene end
                Self::scene_end_process(&mut sp_data, &mut scenes, &mut sc);
                continue;
            }

            let kind = Self::parse_dialog_kind(line);

            let indent_lv = sp_data.parse_indent_lv(line);

            match kind {
                DialogKind::Dialog => {
                    if indent_lv < sp_data.nest_lv {
                        sp_data.nest_lv_count_down(indent_lv);
                        sp_data.truncate_idxs(indent_lv);
                    }

                    sp_data.dialog_count_up();

                    // dialog push
                    let target_dialogs = sc.inner_dialogs_as_mut(&mut sp_data.inner_scene_idxs());
                    target_dialogs.push(Self::dialog_process(line, &sp_data));
                }
                DialogKind::Command => {
                    if indent_lv < sp_data.nest_lv {
                        sp_data.nest_lv_count_down(indent_lv);
                        sp_data.truncate_idxs(indent_lv);
                    }

                    // command push
                    let target_dialogs = sc.inner_dialogs_as_mut(&mut sp_data.inner_scene_idxs());
                    target_dialogs.push(Self::command_process(line));
                }
                DialogKind::Choices => {
                    let is_choices_parent = indent_lv >= sp_data.nest_lv;
                    if is_choices_parent {
                        sp_data.dialog_count_up();
                        sp_data.nest_lv_count_up();
                    } else {
                        sp_data.truncate_idxs(indent_lv);
                    }

                    sp_data.choice_idx_count_up();

                    let mut idxs = sp_data.inner_scene_idxs();
                    if is_choices_parent {
                        // choices parent push
                        let target_dialogs = sc.inner_parent_dialogs_as_mut(&mut idxs.clone());
                        target_dialogs.push(Self::choices_parent_process(&sp_data));
                    }

                    // choice push
                    let target_choice = sc.inner_choices_as_mut(&mut idxs);
                    let cd = Self::choices_child_process(line, &sp_data);
                    target_choice.args.push(DialogBody::Choice(cd));
                }
            }

            // Self::debug_print(line, &sp_data);
        }

        if !sc.dialogs.is_empty() {
            Self::scene_end_process(&mut sp_data, &mut scenes, &mut sc);
        }

        scenes
    }

    fn header_process(line: &str, sp_data: &mut SceneProcessData, sc: &mut Scene) {
        // if line text is "+++", end header section.
        if Self::is_header_symbol(line) {
            let s = &sp_data.header_str;
            sp_data.meta_data.parse(s);
            sp_data.is_header = false;

            if !sp_data.meta_data.title.is_empty() {
                sc.title = sp_data.meta_data.title.clone();
            }
            return;
        }

        sp_data.header_str.push_str(line);
        sp_data.header_str.push('\n');
    }

    fn dialog_process(line: &str, sp_data: &SceneProcessData) -> Dialog {
        let mut iter = line.splitn(2, ':');

        let s0 = iter.next().unwrap_or("").trim();
        let s1 = iter.next().unwrap_or("").trim();

        let talker = if !s1.is_empty() { s0 } else { "unknown" };
        let body = vec![DialogBody::Text(String::from(if !s1.is_empty() {
            s1
        } else {
            s0
        }))];

        let id = format!("{}_{}", sp_data.gen_dialog_label(), talker);

        Dialog::from_dialog_data(DialogKind::Dialog, id, body)
    }

    fn command_process(line: &str) -> Dialog {
        let cmd_str = line.splitn(2, '$').nth(1).unwrap_or("").trim();

        let mut iter = cmd_str.split_whitespace();

        let id = String::from(iter.nth(0).unwrap_or(""));
        let args: Vec<DialogBody> = iter
            .map(|s| {
                let body = String::from(s);
                DialogBody::Text(body)
            })
            .collect();

        Dialog::from_dialog_data(DialogKind::Command, id, args)
    }

    fn choices_parent_process(sp_data: &SceneProcessData) -> Dialog {
        let label = sp_data.gen_dialog_label();
        let s = label.rsplitn(2, 'L').last().unwrap_or("Choices");
        Dialog::from_dialog_data(DialogKind::Choices, s, Vec::new())
    }

    fn choices_child_process(line: &str, sp_data: &SceneProcessData) -> ChoiceData {
        let (_, label) = line.trim_start().split_at(1);
        let id = sp_data.gen_dialog_label();

        ChoiceData::from_texts(id, label.trim())
    }

    fn scene_end_process(
        sp_data: &mut SceneProcessData,
        scenes: &mut Vec<Scene>,
        current_scene: &mut Scene,
    ) {
        scenes.push(current_scene.clone());
        sp_data.reset();
        current_scene.reset();
    }

    fn is_header_symbol(line: &str) -> bool {
        let mut is_header = true;
        let mut chars = line.trim_start().chars();
        for _ in 0..3 {
            match chars.next() {
                Some(c) if c == '+' => {}
                _ => {
                    is_header = false;
                    break;
                }
            }
        }
        is_header
    }

    fn is_scene_end_symbol(line: &str) -> bool {
        let mut is_end_symbol = true;
        let mut chars = line.trim_start().chars();
        for _ in 0..3 {
            match chars.next() {
                Some(c) if c == '=' => {}
                _ => {
                    is_end_symbol = false;
                    break;
                }
            }
        }
        is_end_symbol
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

    fn parse_dialog_kind(line: &str) -> DialogKind {
        // '\0' is placeholder char
        let c = line.trim_start().chars().nth(0).unwrap_or('\0');

        match c {
            '$' => DialogKind::Command,
            '*' | '+' | '-' => DialogKind::Choices,
            _ => DialogKind::Dialog,
        }
    }

    // fn debug_print<T: AsRef<str>>(line: T, sp_data: &SceneProcessData) {
    //     let print_str: String = line.as_ref().chars().take(16).collect();
    //
    //     println!(
    //         "#{}: {:?} {} // {}",
    //         sp_data.line_cnt,
    //         sp_data.inner_scene_idxs(),
    //         print_str,
    //         sp_data.gen_dialog_label()
    //     );
    // }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SceneProcessData {
    line_cnt: usize,
    indent_cnts: Vec<usize>,
    pub dialog_idxs: Vec<usize>,
    pub nest_lv: usize,
    pub is_header: bool,
    pub choice_idxs: Vec<(usize, usize)>,
    pub header_str: String,
    pub meta_data: MetaData,
}

impl Default for SceneProcessData {
    fn default() -> Self {
        Self {
            dialog_idxs: Vec::new(),
            nest_lv: 0,
            line_cnt: 0,
            is_header: false,
            indent_cnts: Vec::new(),
            choice_idxs: Vec::new(),
            header_str: String::new(),
            meta_data: MetaData::new(),
        }
    }
}

impl SceneProcessData {
    pub fn new() -> Self {
        SceneProcessData::default()
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn dialog_count_up(&mut self) {
        let idxs = &mut self.dialog_idxs;
        if self.nest_lv < idxs.len() {
            idxs[self.nest_lv] += 1;
        } else {
            idxs.push(0);
        }
    }

    pub fn choice_idx_count_up(&mut self) {
        let idxs = &mut self.choice_idxs;
        let past_nest_lv = idxs.len();

        if self.nest_lv == 0 {
            return;
        }

        let i = self.nest_lv - 1;

        idxs.truncate(self.nest_lv);

        if past_nest_lv > self.nest_lv {
            let (ci, li) = &mut idxs[i];
            *ci += 1;
            *li = 0;
        } else if i < idxs.len() {
            let (_, li) = &mut idxs[i];
            *li += 1;
        } else {
            idxs.push((0, 0));
        }
    }

    pub fn truncate_idxs(&mut self, indent_lv: usize) {
        let l = indent_lv + 1;
        self.dialog_idxs.truncate(l);
        self.indent_cnts.truncate(l);
    }

    pub fn nest_lv_count_up(&mut self) {
        self.nest_lv += 1;
    }

    pub fn nest_lv_count_down(&mut self, indent_lv: usize) {
        // let l = indent_lv + 1;
        self.nest_lv = indent_lv;
    }

    pub fn line_count_up(&mut self) {
        self.line_cnt += 1;
    }

    // return: [di, ci, li, di, ci, li...] if nest_lv > 0
    //       : [] if nest_lv == 0
    // where : di = dialog_idx, ci = choice_idx, li = choice_label_idx
    pub fn inner_scene_idxs(&self) -> Vec<usize> {
        self.dialog_idxs
            .iter()
            .zip(self.choice_idxs.iter())
            .take(self.nest_lv)
            .flat_map(|(di, (ci, li))| vec![*di, *ci, *li])
            .collect()
    }

    fn latest_indent_cnt(&self) -> usize {
        *self.indent_cnts.last().unwrap_or(&0)
    }

    pub fn parse_indent_lv(&mut self, line: &str) -> usize {
        let indent_cnt = Self::count_indent_chars(line);

        let latest_cnt = self.latest_indent_cnt();
        if indent_cnt > latest_cnt {
            self.indent_cnts.push(indent_cnt);
            self.indent_cnts.len()
        } else {
            let pos = self.indent_cnts.iter().rposition(|&x| indent_cnt >= x);

            match pos {
                // adapt range to self.indent_cnts.len()
                Some(n) => n + 1,
                None => 0,
            }
        }
    }

    pub fn gen_dialog_label(&self) -> String {
        let mut s = self.meta_data.title.clone();

        (0..self.dialog_idxs.len()).for_each(|i| {
            let dialog_idx = self.dialog_idxs[i];
            s.push_str(&format!("_{}", dialog_idx + 1));

            if i < self.choice_idxs.len() {
                let (ci, li) = self.choice_idxs[i];
                s.push_str(&format!("_C{}L{}", ci + 1, li + 1));
            }
        });

        s
    }

    pub fn count_indent_chars(line: &str) -> usize {
        line.chars()
            .position(|c| c != ' ' && c != '\u{0009}')
            .unwrap_or(0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(default)]
pub struct MetaData {
    pub title: String,
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            title: String::from("UnknownScene"),
        }
    }
}

impl MetaData {
    pub fn new() -> Self {
        MetaData::default()
    }

    pub fn parse<T: AsRef<str>>(&mut self, toml_str: T) {
        let data: MetaData = match toml::from_str(toml_str.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("KukuriScript toml parse error: {:?}", e);
                Self::new()
            }
        };
        *self = data
    }
}

#[cfg(test)]
mod tests {
    use super::{KukuriScript, SceneProcessData};
    use crate::core::dialog::DialogKind;

    #[test]
    fn test_parse_dialog_kind() {
        let tests = [
            ("$ jump ttt", DialogKind::Command),
            ("A: test dialog", DialogKind::Dialog),
            ("non-talker dialog", DialogKind::Dialog),
            ("  # commented line", DialogKind::Dialog),
            ("* choice1", DialogKind::Choices),
            ("+ choice2", DialogKind::Choices),
            ("- choice3", DialogKind::Choices),
        ];

        for &(src, expected) in &tests {
            assert_eq!(expected, KukuriScript::parse_dialog_kind(src))
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
    fn test_is_header_symbol() {
        let tests = [
            ("+++", true),
            ("++++++", true),
            ("++not header symbol++", false),
            ("   +++ untrimmed line", true),
            ("not+++header symbol", false),
        ];

        for &(src, expected) in &tests {
            assert_eq!(expected, KukuriScript::is_header_symbol(src))
        }
    }

    #[test]
    fn test_is_scene_end_symbol() {
        let tests = [
            ("===", true),
            ("==not scene end==", false),
            ("========", true),
            ("   === untrimmed line", true),
            ("not===scene end", false),
        ];

        for &(src, ref expected) in &tests {
            assert_eq!(expected, &KukuriScript::is_scene_end_symbol(src));
        }
    }

    #[test]
    fn test_dialog_count_up() {
        let mut sp_data = SceneProcessData::new();

        let tests = [Vec::new(), vec![0], vec![1], vec![2], vec![3], vec![4]];

        for expected in &tests {
            assert_eq!(*expected, sp_data.dialog_idxs);
            sp_data.dialog_count_up();
        }

        sp_data.nest_lv_count_up();
        let tests2 = [vec![5], vec![5, 0], vec![5, 1], vec![5, 2]];
        for expected in &tests2 {
            assert_eq!(*expected, sp_data.dialog_idxs);
            sp_data.dialog_count_up();
        }

        sp_data.nest_lv_count_down(0);
        sp_data.truncate_idxs(0);
        let tests3 = [vec![5], vec![6], vec![7], vec![8]];
        for expected in &tests3 {
            assert_eq!(*expected, sp_data.dialog_idxs);
            sp_data.dialog_count_up();
        }
    }

    #[test]
    fn test_count_indent_chars() {
        let tests = [
            ("", 0),
            ("A: space 0", 0),
            ("  A: space 2", 2),
            ("    A: space 4", 4),
            ("	A: tab1", 1),
            ("		A: tab2", 2),
        ];

        for &(src, expected) in &tests {
            assert_eq!(expected, SceneProcessData::count_indent_chars(src));
        }
    }

    #[test]
    fn test_latest_index_cnt() {
        let mut sp_data = SceneProcessData::new();
        assert_eq!(0, sp_data.latest_indent_cnt());

        sp_data.indent_cnts.push(4);
        assert_eq!(4, sp_data.latest_indent_cnt());

        sp_data.indent_cnts.push(8);
        assert_eq!(8, sp_data.latest_indent_cnt());

        sp_data.indent_cnts.remove(0);
        assert_eq!(8, sp_data.latest_indent_cnt());

        sp_data.indent_cnts.clear();
        assert_eq!(0, sp_data.latest_indent_cnt());
    }

    #[test]
    fn test_parse_indent_lv() {
        let mut sp_data = SceneProcessData::new();
        [
            ("foobar", 0),
            ("  foobar", 1),
            ("    foobar", 2),
            ("  foobar", 1),
            ("foobar", 0),
        ]
        .iter()
        .for_each(|&(src, expected)| {
            // println!("test_parse_indent_lv: {}", sp_data.latest_indent_cnt());
            assert_eq!(expected, sp_data.parse_indent_lv(src));
        });
    }

    #[test]
    fn test_gen_dialog_label() {
        let mut sp_data = SceneProcessData::new();
        sp_data.dialog_idxs.push(5);
        sp_data.choice_idxs.push((2, 3));
        assert_eq!("UnknownScene_6_C3L4", sp_data.gen_dialog_label());

        sp_data.dialog_idxs.push(2);
        sp_data.choice_idxs.push((8, 5));
        assert_eq!("UnknownScene_6_C3L4_3_C9L6", sp_data.gen_dialog_label());

        sp_data.dialog_idxs.clear();
        sp_data.dialog_idxs.push(2);
        assert_eq!("UnknownScene_3_C3L4", sp_data.gen_dialog_label());

        sp_data.choice_idxs.clear();
        assert_eq!("UnknownScene_3", sp_data.gen_dialog_label());
    }

    #[test]
    fn test_choice_idx_count_up() {
        let mut sp_data = SceneProcessData::new();
        sp_data.choice_idx_count_up();
        assert_eq!(Vec::<(usize, usize)>::new(), sp_data.choice_idxs);

        sp_data.nest_lv_count_up();
        sp_data.choice_idx_count_up();
        assert_eq!(vec![(0, 0)], sp_data.choice_idxs);
        sp_data.choice_idx_count_up();
        assert_eq!(vec![(0, 1)], sp_data.choice_idxs);

        sp_data.nest_lv_count_up();
        sp_data.choice_idx_count_up();
        assert_eq!(vec![(0, 1), (0, 0)], sp_data.choice_idxs);
        sp_data.choice_idx_count_up();
        assert_eq!(vec![(0, 1), (0, 1)], sp_data.choice_idxs);

        sp_data.nest_lv_count_down(1);
        sp_data.choice_idx_count_up();
        assert_eq!(vec![(1, 0)], sp_data.choice_idxs);
    }

    #[test]
    fn test_inner_scene_idxs() {
        let mut sp_data = SceneProcessData::new();
        assert_eq!(Vec::<usize>::new(), sp_data.inner_scene_idxs());

        sp_data.nest_lv = 1;
        sp_data.dialog_idxs.push(5);
        sp_data.choice_idxs.push((2, 3));
        assert_eq!(vec![5, 2, 3], sp_data.inner_scene_idxs());

        sp_data.nest_lv = 2;
        sp_data.dialog_idxs.push(8);
        sp_data.choice_idxs.push((5, 2));
        assert_eq!(vec![5, 2, 3, 8, 5, 2], sp_data.inner_scene_idxs());

        sp_data.nest_lv = 0;
        assert_eq!(Vec::<usize>::new(), sp_data.inner_scene_idxs());
    }
}
