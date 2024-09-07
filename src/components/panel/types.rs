use serde::{Deserialize, Serialize, };
use crate::components::content_panel::types::ContentfulContentPanel;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "typename")]
pub enum ContentfulPanel {
    ContentPanel(ContentfulContentPanel)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPanelsCollection {
    pub items: Vec<ContentfulPanel>,
}