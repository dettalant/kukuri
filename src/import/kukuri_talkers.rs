use crate::core::{kukuri_data::KukuriData, talker::Talker};

pub struct KukuriTalkers;
impl KukuriTalkers {
    pub fn parse(content: &str) -> KukuriData {
        let mut talkers: Vec<Talker> = Vec::new();

        for full_line in content.lines() {
            let line = Self::trim_comment(full_line);
            if line.trim().is_empty() {
                continue;
            }

            if let Some(talker) = Self::line_parse(line) {
                talkers.push(talker);
            }
        }

        KukuriData::from_talkers(talkers)
    }

    fn line_parse(line: &str) -> Option<Talker> {
        match line.find(':') {
            Some(i) => {
                let (id, _) = line.split_at(i);
                let (_, name) = line.split_at(i + 1);
                Some(Talker::from_strs(id.trim(), name.trim()))
            }
            None => None,
        }
    }

    fn trim_comment(line: &str) -> &str {
        let mut bs_i = 0;
        let symbol_idx = line.char_indices().find(|&(i, c)| {
            // match backslash char
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
}

#[cfg(test)]
mod tests {
    use super::KukuriTalkers;
    use crate::core::{kukuri_data::KukuriData, talker::Talker};

    #[test]
    fn test_parse() {
        let talker_src = r#"
A : Alpha
 B: Bravo
 C  :  Charlie
"#;
        let expected = KukuriData::from_talkers(vec![
            Talker::from_strs("A", "Alpha"),
            Talker::from_strs("B", "Bravo"),
            Talker::from_strs("C", "Charlie"),
        ]);

        assert_eq!(expected, KukuriTalkers::parse(talker_src));
    }
}
