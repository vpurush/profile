use serde::{Deserialize, Serialize};
use crate::contentful_richtext_renderer::types::ContentfulRichTextContent;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulContentPanel {
    // pub typename: String,
    pub title: String,
    pub content: ContentfulRichTextContent,
}