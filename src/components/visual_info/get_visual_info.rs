use serde::{Deserialize, Serialize};
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::visual_info::types::{ContentfulVisualInfo, VisualInfo};

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

pub fn visual_info_decorator(contentful_visual_info: ContentfulVisualInfo) -> VisualInfo {
    VisualInfo {
        text_alignment: contentful_visual_info.text_alignment.into_iter().next().unwrap(),
        background_color: contentful_visual_info.background_color.into_iter().next().unwrap(),
    }
}