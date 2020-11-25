use serde::{Deserialize, Serialize};

use super::dialog::Scene;
use super::talker::Talker;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum KukuriData {
    Scenes(Vec<Scene>),
    Talkers(Vec<Talker>),
}

impl KukuriData {
    pub fn new() -> Self {
        Self::Scenes(Vec::new())
    }

    pub fn from_scenes(scenes: Vec<Scene>) -> Self {
        Self::Scenes(scenes)
    }

    pub fn from_talkers(talkers: Vec<Talker>) -> Self {
        Self::Talkers(talkers)
    }
}
