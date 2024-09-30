use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::visual_info::get_visual_info::visual_info_decorator;
use serde::{Deserialize, Serialize};

pub fn get_content_panel_query() -> String {
    format!(
        "\
        ... on ContentPanel {{
          {}
        }}
    ",
        get_content_panel_single_reference_query()
    )
}
pub fn get_content_panel_single_reference_query() -> String {
    String::from(
        "\
      title
      content {
        json
      }            
      visualInfo {
        textAlignment
        backgroundColor
      }
    ",
    )
}

pub fn content_panel_decorator(contentful_content_panel: ContentfulContentPanel) -> ContentPanel {
    ContentPanel {
        title: contentful_content_panel.title,
        content: contentful_content_panel.content,
        visual_info: visual_info_decorator(contentful_content_panel.visual_info),
    }
}
