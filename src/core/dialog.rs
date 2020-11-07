#[derive(Clone, Debug, PartialEq)]
pub enum DialogKind {
    Dialog,
    Command,
    Choices,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DialogBody {
    // Text(DialogBody || CommandArg)
    Text(String),
    // Choice(ChoiceLabel, InnerDialog)
    // Choice(String, Vec<Dialog>),
}

#[derive(Clone, Debug)]
pub struct Dialog {
    pub kind: DialogKind,
    // if DialogKind::Dialog => DialogTalker
    // if DialogKind::Command => CommandName
    // if DialogKind::Choices => "choices" // placeholder
    pub id: String,
    pub args: Vec<DialogBody>
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
        Dialog {
            kind,
            id,
            args
        }
    }
}

#[derive(Debug)]
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
        Scene {
            title,
            dialogs,
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            title: String::new(),
            dialogs: Vec::new()
        }
    }
}
