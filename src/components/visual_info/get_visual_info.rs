use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::visual_info::types::ContentfulVisualInfoTextAlign::Left;
use crate::components::visual_info::types::{ContentfulVisualInfo, ContentfulVisualInfoBackground, VisualInfo};
use serde::{Deserialize, Serialize};

pub fn get_content_panel_query() -> String {
    String::from(
        "\
        ... on ContentPanel{
          title
          content {
            json
          }
        }
    ",
    )
}

pub fn visual_info_decorator(contentful_visual_info: Option<ContentfulVisualInfo>) -> VisualInfo {
    match contentful_visual_info {
        Some(contentful_visual_info_some) => VisualInfo {
            text_alignment: contentful_visual_info_some
                .text_alignment
                .into_iter()
                .next()
                .unwrap(),
            background_color: contentful_visual_info_some
                .background_color
                .into_iter()
                .next()
                .unwrap(),
        },
        None => VisualInfo {
            text_alignment: Left,
            background_color: ContentfulVisualInfoBackground::None,
        },
    }
}
