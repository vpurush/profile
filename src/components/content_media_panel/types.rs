use serde::{Deserialize, Serialize};
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::picture_panel::types::{ContentfulPicturePanel, PicturePanel};
use crate::components::visual_info::types::{ContentfulVisualInfo, VisualInfo};
use crate::contentful_richtext_renderer::types::ContentfulRichTextContent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentMediaPanelVariants {
    #[serde(rename(serialize = "left", deserialize = "left"))]
    Left,
    #[serde(rename(serialize = "right", deserialize = "right"))]
    Right
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulContentMediaPanel {
    pub content: ContentfulContentPanel,
    pub media: ContentfulPicturePanel,
    pub variant: ContentMediaPanelVariants,
    #[serde(rename(serialize = "visualInfo", deserialize = "visualInfo"))]
    pub visual_info: Option<ContentfulVisualInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentMediaPanel {
    pub content: ContentPanel,
    pub media: PicturePanel,
    pub variant: ContentMediaPanelVariants,
    pub visual_info: VisualInfo
}