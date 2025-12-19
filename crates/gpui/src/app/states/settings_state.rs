use gpui::Global;
use serde::{Deserialize, Serialize};

use crate::domain::entities::settings::DbContext;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    contexts: Vec<DbContext>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            contexts: Vec::new(),
        }
    }
}

impl Global for Settings {}
