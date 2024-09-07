use serde::{Deserialize, Serialize};
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::visual_info::get_visual_info::visual_info_decorator;

pub fn get_content_panel_query() -> String {
    String::from("\
        ... on ContentPanel{
          title
          content {
            json
          }            
          visualInfo {
            textAlignment
            backgroundColor
          }
        }
    ")
}

pub fn content_panel_decorator(contentful_content_panel: ContentfulContentPanel) -> ContentPanel {
    ContentPanel {
        title: contentful_content_panel.title,
        content: contentful_content_panel.content,
        visual_info: visual_info_decorator(contentful_content_panel.visual_info)
    }
}