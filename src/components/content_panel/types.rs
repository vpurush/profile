use serde::{Deserialize, Serialize};
use crate::components::visual_info::types::{ContentfulVisualInfo, VisualInfo};
use crate::contentful_richtext_renderer::types::ContentfulRichTextContent;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulContentPanel {
    pub title: Option<String>,
    pub content: ContentfulRichTextContent,
    #[serde(rename(serialize = "visualInfo", deserialize = "visualInfo"))]
    pub visual_info: Option<ContentfulVisualInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentPanel {
    pub title: Option<String>,
    pub content: ContentfulRichTextContent,
    pub visual_info: VisualInfo
}