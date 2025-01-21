use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatus {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed
}

impl fmt::Display for QuestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestStatus::Open => write!(f, "open"),
            QuestStatus::InJourney => write!(f, "InJourney"),
            QuestStatus::Completed => write!(f, "Completed"),
            QuestStatus::Failed => write!(f, "Failed"),
        }
    }
}