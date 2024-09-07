use serde::{Deserialize, Serialize};
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};

pub fn get_content_panel_query() -> String {
    String::from("\
        ... on ContentPanel{
          title
          content {
            json
          }
        }
    ")
}

pub fn content_panel_decorator(contentful_content_panel: ContentfulContentPanel) -> ContentPanel {
    ContentPanel {
        title: contentful_content_panel.title,
        content: contentful_content_panel.content
    }
}