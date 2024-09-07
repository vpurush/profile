use serde::{Deserialize, Serialize};
use crate::contentful_richtext_renderer::types::ContentfulRichTextContent;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulContentPanel {
    pub title: String,
    pub content: ContentfulRichTextContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentPanel {
    pub title: String,
    pub content: ContentfulRichTextContent,
}