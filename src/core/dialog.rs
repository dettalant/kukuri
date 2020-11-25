use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DialogKind {
    Dialog,
    Command,
    Choices,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub enum DialogBody {
    // Text(DialogBody || CommandArg)
    Text(String),
    Choice(ChoiceData),
}

impl DialogBody {
    pub fn gen_text<T: AsRef<str>>(s: T) -> Self {
        Self::Text(String::from(s.as_ref()))
    }
}

impl Serialize for DialogBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            // flatten serialize output
            Self::Text(ref s) => serializer.serialize_str(s),
            Self::Choice(ref cd) => serializer.serialize_newtype_struct("ChoiceData", cd),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct Dialog {
    pub kind: DialogKind,
    // if DialogKind::Dialog => Dialog id e.g. "SceneTitle_idx_talker"
    // if DialogKind::Command => CommandName
    // if DialogKind::Choices => Choices id e.g. "SceneTitle_idx_C1"
    pub id: String,
    pub args: Vec<DialogBody>,
}

impl Dialog {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Dialog {
            kind: DialogKind::Dialog,
            id: String::new(),
            args: Vec::new(),
        }
    }

    pub fn from_dialog_data<T: AsRef<str>>(kind: DialogKind, id: T, args: Vec<DialogBody>) -> Self {
        Dialog {
            kind,
            id: String::from(id.as_ref()),
            args,
        }
    }
}

impl Serialize for Dialog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let is_exclude_orig_text =
            std::env::var("KUKURI_IS_EXCLUDE_ORIG_TEXT").unwrap_or(String::from("FALSE")) == "TRUE"
                && self.kind == DialogKind::Dialog;
        let mut ss = serializer.serialize_struct("Dialog", 3)?;
        ss.serialize_field("id", &self.id)?;
        ss.serialize_field("kind", &self.kind)?;

        let args = if is_exclude_orig_text {
            self.args.iter().skip(1).cloned().collect()
        } else {
            self.args.clone()
        };
        ss.serialize_field("args", &args)?;

        ss.end()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct ChoiceData {
    // Choice id e.g. "SceneTitle_1_C1L2"
    pub id: String,
    // Choice label text
    pub label: String,
    // Choice inner dialogs
    pub dialogs: Vec<Dialog>,
}

impl Default for ChoiceData {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            dialogs: Vec::new(),
        }
    }
}

impl ChoiceData {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChoiceData::default()
    }

    pub fn from_texts<T: AsRef<str>, T2: AsRef<str>>(id: T, label: T2) -> Self {
        Self {
            id: String::from(id.as_ref()),
            label: String::from(label.as_ref()),
            ..Default::default()
        }
    }
}

impl Serialize for ChoiceData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let is_exclude_orig_text =
            std::env::var("KUKURI_IS_EXCLUDE_ORIG_TEXT").unwrap_or(String::from("FALSE")) == "TRUE";
        let s_len = if is_exclude_orig_text { 2 } else { 3 };
        let mut ss = serializer.serialize_struct("ChoiceData", s_len)?;
        ss.serialize_field("id", &self.id)?;

        if !is_exclude_orig_text {
            ss.serialize_field("label", &self.label)?;
        }

        ss.serialize_field("dialogs", &self.dialogs)?;
        ss.end()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Scene {
    // SceneTitle
    pub title: String,
    pub dialogs: Vec<Dialog>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    // pub fn from_title<T: AsRef<str>>(title: T) -> Self {
    //     Scene {
    //         title: String::from(title.as_ref()),
    //         ..Default::default()
    //     }
    // }

    // pub fn from_scene_data<T: AsRef<str>>(title: T, dialogs: Vec<Dialog>) -> Self {
    //     Scene {
    //         title: String::from(title.as_ref()),
    //         dialogs,
    //     }
    // }

    pub fn inner_dialogs_as_mut(&mut self, inner_scene_idxs: &mut Vec<usize>) -> &mut Vec<Dialog> {
        Self::retrieve_inner_dialogs_as_mut(&mut self.dialogs, inner_scene_idxs)
    }

    pub fn inner_parent_dialogs_as_mut(
        &mut self,
        inner_scene_idxs: &mut Vec<usize>,
    ) -> &mut Vec<Dialog> {
        // truncate length
        let truncate_idxs = Self::truncate_scene_idxs(inner_scene_idxs);

        Self::retrieve_inner_dialogs_as_mut(&mut self.dialogs, truncate_idxs)
    }

    fn retrieve_inner_dialogs_as_mut<'a>(
        dialogs: &'a mut Vec<Dialog>,
        inner_scene_idxs: &mut Vec<usize>,
    ) -> &'a mut Vec<Dialog> {
        if inner_scene_idxs.is_empty() {
            return dialogs;
        }

        let v: Vec<usize> = inner_scene_idxs.drain(..3).collect();
        let di = v[0];
        let li = v[2];
        let choices = &mut dialogs[di];
        if let DialogBody::Choice(ref mut cd) = choices.args[li] {
            return Self::retrieve_inner_dialogs_as_mut(&mut cd.dialogs, inner_scene_idxs);
        } else {
            panic!(
                "get_inner_dialogs_as_mut: dialog_body {} is not choice!",
                li
            );
        }
    }

    pub fn inner_choices_as_mut(&mut self, inner_scene_idxs: &mut Vec<usize>) -> &mut Dialog {
        let ci_i = inner_scene_idxs.len() - 2;
        let ci = inner_scene_idxs[ci_i];

        let truncate_idxs = Self::truncate_scene_idxs(inner_scene_idxs);

        // retrieve a parent dialogs
        let dialogs = Self::retrieve_inner_dialogs_as_mut(&mut self.dialogs, truncate_idxs);
        let v = dialogs.clone();

        dialogs
            .iter_mut()
            .filter(|d| d.kind == DialogKind::Choices)
            .nth(ci)
            .expect(&format!(
                "inner_choices_as_mut: Unable to find a choices dialog {}, {:#?}",
                ci, v
            ))
    }

    pub fn truncate_scene_idxs(scene_idxs: &mut Vec<usize>) -> &mut Vec<usize> {
        let l = if scene_idxs.len() >= 3 {
            scene_idxs.len() - 3
        } else {
            0
        };
        scene_idxs.truncate(l);
        scene_idxs
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            title: String::new(),
            dialogs: Vec::new(),
        }
    }
}

// scene_title: scene_dialogs
pub type Scenes = HashMap<String, Vec<Dialog>>;

#[cfg(test)]
mod tests {
    use super::{ChoiceData, Dialog, DialogBody, DialogKind, Scene};

    fn gen_test_scene() -> Scene {
        let s = String::new();
        let inner_dialogs = vec![
            Dialog::new(),
            Dialog::from_dialog_data(
                DialogKind::Dialog,
                String::from("innerdialog_0"),
                Vec::new(),
            ),
        ];

        let inner_choices = Dialog {
            kind: DialogKind::Choices,
            id: s.clone(),
            args: vec![
                DialogBody::Choice(ChoiceData::new()),
                DialogBody::Choice(ChoiceData {
                    dialogs: inner_dialogs.clone(),
                    ..Default::default()
                }),
            ],
        };

        let choices = Dialog {
            kind: DialogKind::Choices,
            id: s.clone(),
            args: vec![DialogBody::Choice(ChoiceData {
                dialogs: vec![Dialog::new(), Dialog::new(), Dialog::new(), inner_choices],
                ..Default::default()
            })],
        };

        Scene {
            title: String::from(""),
            dialogs: vec![Dialog::new(), Dialog::new(), choices],
        }
    }

    #[test]
    fn test_inner_dialogs_as_mut() {
        let mut inner_dialogs = vec![
            Dialog::new(),
            Dialog::from_dialog_data(
                DialogKind::Dialog,
                String::from("innerdialog_0"),
                Vec::new(),
            ),
        ];

        let mut scene = gen_test_scene();

        assert_eq!(
            &mut inner_dialogs,
            scene.inner_dialogs_as_mut(&mut vec![2, 0, 0, 3, 0, 1])
        )
    }

    #[test]
    fn test_truncate_scene_idxs() {
        [
            (vec![5, 0, 0], Vec::new()),
            (vec![1, 0, 0, 3, 0, 2], vec![1, 0, 0]),
            (vec![3, 1, 3, 5, 2, 1, 8, 3, 2], vec![3, 1, 3, 5, 2, 1]),
        ]
        .iter_mut()
        .for_each(|(src, expected)| assert_eq!(expected, Scene::truncate_scene_idxs(src)))
    }

    #[test]
    fn test_inner_parent_dialogs_as_mut() {
        let mut scene = gen_test_scene();
        let mut scene2 = scene.clone();
        assert_eq!(
            &mut scene.dialogs.clone(),
            scene.inner_parent_dialogs_as_mut(&mut vec![2, 0, 0])
        );

        match scene.dialogs[2].args[0] {
            DialogBody::Choice(ref mut cd) => assert_eq!(
                &mut cd.dialogs,
                scene2.inner_parent_dialogs_as_mut(&mut vec![2, 0, 0, 3, 0, 1])
            ),
            _ => panic!("Unable to find ChoiceData"),
        };
    }

    #[test]
    fn test_inner_choices_as_mut() {
        let s = String::new();

        let inner_dialogs = vec![
            Dialog::new(),
            Dialog::from_dialog_data(
                DialogKind::Dialog,
                String::from("innerdialog_0"),
                Vec::new(),
            ),
        ];

        let mut inner_choices = Dialog {
            kind: DialogKind::Choices,
            id: s.clone(),
            args: vec![
                DialogBody::Choice(ChoiceData::new()),
                DialogBody::Choice(ChoiceData {
                    dialogs: inner_dialogs.clone(),
                    ..Default::default()
                }),
            ],
        };

        let mut choices = Dialog {
            kind: DialogKind::Choices,
            id: s.clone(),
            args: vec![DialogBody::Choice(ChoiceData {
                dialogs: vec![
                    Dialog::new(),
                    Dialog::new(),
                    Dialog::new(),
                    inner_choices.clone(),
                ],
                ..Default::default()
            })],
        };

        let mut scene = gen_test_scene();

        assert_eq!(&mut choices, scene.inner_choices_as_mut(&mut vec![2, 0, 0]));
        assert_eq!(
            &mut inner_choices,
            scene.inner_choices_as_mut(&mut vec![2, 0, 0, 3, 0, 1])
        )
    }
}
