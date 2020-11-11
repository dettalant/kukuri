use crate::core::dialog::{Dialog, DialogBody, DialogKind, Scene};
use serde::{Deserialize, Serialize};

pub struct KukuriScript;

impl KukuriScript {
    pub fn parse(content: &str) {
        let mut sp_data = SceneProcessData::new();

        for full_line in content.lines() {
            sp_data.line_count_up();
            let line = Self::trim_comment(full_line);

            if line.trim().is_empty() {
                continue;
            };

            if sp_data.is_header {
                Self::header_process(line, &mut sp_data);
                continue;
            }

            if Self::is_header_symbol(line) {
                sp_data.is_header = true;
                continue;
            }

            if Self::is_scene_end_symbol(line) {
                println!("sp_data: {:?}", sp_data);
                sp_data.reset();
                // scene end
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
                }
                DialogKind::Command => {
                    if indent_lv < sp_data.nest_lv {
                        sp_data.nest_lv_count_down(indent_lv);
                        sp_data.truncate_idxs(indent_lv);
                    }
                }
                DialogKind::Choices => {
                    if indent_lv >= sp_data.nest_lv {
                        sp_data.dialog_count_up();
                        sp_data.nest_lv_count_up();
                    } else {
                        sp_data.truncate_idxs(indent_lv);
                    }

                    sp_data.choice_idx_count_up();
                }
            }

            Self::debug_print(line, &sp_data);
        }

        // after end of line
        println!("sp_data: {:?}", sp_data);
        sp_data.reset();
    }

    fn header_process(line: &str, sp_data: &mut SceneProcessData) {
        // if line text is "+++", end header section.
        if Self::is_header_symbol(line) {
            let s = &sp_data.header_str;
            sp_data.meta_data.parse(s);
            sp_data.is_header = false;
            return;
        }

        sp_data.header_str.push_str(line);
        sp_data.header_str.push('\n');
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

    fn debug_print<T: AsRef<str>>(line: T, sp_data: &SceneProcessData) {
        let print_str: String = line.as_ref().chars().take(16).collect();

        println!(
            "#{}: {:?} {} // {}",
            sp_data.line_cnt,
            sp_data.dialog_idxs,
            print_str,
            sp_data.gen_dialog_label()
        );
    }
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

    // pub fn meta_idx_count_up(&mut self) {
    //     let idxs = &mut self.meta_idxs;
    //     if self.nest_lv < idxs.len() {
    //         idxs[self.nest_lv] += 1;
    //     } else {
    //         idxs.push(0);
    //     }
    // }

    pub fn choice_idx_count_up(&mut self) {
        let idxs = &mut self.choice_idxs;
        let past_nest_lv = idxs.len();
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

    fn latest_indent_cnt(&self) -> usize {
        let l = self.indent_cnts.len();
        if l > 0 {
            self.indent_cnts[l - 1]
        } else {
            0
        }
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

        let tests = [Vec::new(), vec![1], vec![2], vec![3], vec![4], vec![5]];

        for expected in &tests {
            assert_eq!(*expected, sp_data.dialog_idxs);
            sp_data.dialog_count_up()
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
}
