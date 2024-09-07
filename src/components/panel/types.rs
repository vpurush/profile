use serde::{Deserialize, Serialize, };
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "typename")]
pub enum ContentfulPanel {
    ContentPanel(ContentfulContentPanel)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPanelsCollection {
    pub items: Vec<ContentfulPanel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Panel {
    ContentPanel(ContentPanel)
}
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct PanelsCollection {
//     pub items: Vec<Panel>,
// }