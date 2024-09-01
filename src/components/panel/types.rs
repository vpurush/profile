use serde::{Deserialize, Serialize, };
use crate::components::content_panel::types::ContentPanel;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "typename")]
pub enum Panel {
    ContentPanel(ContentPanel)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PanelsCollection {
    pub items: Vec<Panel>,
}