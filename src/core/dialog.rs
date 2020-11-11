#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DialogKind {
    Dialog,
    Command,
    Choices,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DialogBody {
    // Text(DialogBody || CommandArg)
    Text(String),
    // Choice(ChoiceLabel, InnerDialog)
    // Choice(String, Vec<Dialog>),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dialog {
    pub kind: DialogKind,
    // if DialogKind::Dialog => Dialog label e.g. "SceneTitle_idx_talker"
    // if DialogKind::Command => CommandName
    // if DialogKind::Choices => Choice label e.g. "SceneTitle_idx_C1L2"
    pub id: String,
    pub args: Vec<DialogBody>,
}

impl Dialog {
    // pub fn new() -> Self {
    //     Dialog {
    //         kind: DialogKind::Dialog,
    //         id: String::new(),
    //         args: Vec::new()
    //     }
    // }

    pub fn from_dialog_data(kind: DialogKind, id: String, args: Vec<DialogBody>) -> Self {
        Dialog { kind, id, args }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Scene {
    // SceneTitle
    pub title: String,
    pub dialogs: Vec<Dialog>,
}

impl Scene {
    // pub fn new() -> Self {
    //     Scene {
    //         ..Default::default()
    //     }
    // }

    // pub fn from_title(title: &str) -> Self {
    //     Scene {
    //         title: String::from(title),
    //         ..Default::default()
    //     }
    // }

    pub fn from_scene_data(title: String, dialogs: Vec<Dialog>) -> Self {
        Scene { title, dialogs }
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
