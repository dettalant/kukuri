use serde::{Deserialize, Serialize};

// Talker(id, name)
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Talker(String, String);

impl Default for Talker {
    fn default() -> Self {
        Self(String::from("unknown"), String::from("unknown"))
    }
}

impl Talker {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Talker::default()
    }

    pub fn from_strs<T: AsRef<str>, T2: AsRef<str>>(id: T, name: T2) -> Self {
        Self(String::from(id.as_ref()), String::from(name.as_ref()))
    }

    pub fn id(&self) -> &str {
        &self.0
    }

    pub fn name(&self) -> &str {
        &self.1
    }
}
