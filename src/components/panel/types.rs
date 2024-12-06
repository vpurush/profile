use serde::{Deserialize, Serialize, };
use crate::components::content_media_panel::types::{ContentMediaPanel, ContentfulContentMediaPanel};
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::picture_panel::types::{ContentfulPicturePanel, PicturePanel};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "typename")]
pub enum ContentfulPanel {
    ContentPanel(ContentfulContentPanel),
    PicturePanel(ContentfulPicturePanel),
    ContentMediaPanel(ContentfulContentMediaPanel),
    SectionMediaPanel,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPanelsCollection {
    pub items: Vec<ContentfulPanel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Panel {
    ContentPanel(ContentPanel),
    PicturePanel(PicturePanel),
    ContentMediaPanel(ContentMediaPanel),
    SectionMediaPanel
}
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct PanelsCollection {
//     pub items: Vec<Panel>,
// }