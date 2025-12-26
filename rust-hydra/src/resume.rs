use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ResumeState {
    pub user_index: usize,
    pub password_line: usize,
    pub global_index: usize,
}

impl ResumeState {
    pub fn save(&self) {
        let _ = fs::write("restore.json", serde_json::to_string(self).unwrap());
    }

    pub fn load() -> Option<Self> {
        fs::read_to_string("restore.json")
            .ok()
            .and_then(|d| serde_json::from_str(&d).ok())
    }
}
