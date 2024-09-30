use crate::components::content_media_panel::types::{ContentMediaPanel, ContentfulContentMediaPanel};
use crate::components::content_panel::get_content_panel::{content_panel_decorator, get_content_panel_single_reference_query};
use crate::components::content_panel::types::{ContentPanel, ContentfulContentPanel};
use crate::components::picture_panel::get_picture_panel::{get_picture_panel_single_reference_query, picture_panel_decorator};
use crate::components::visual_info::get_visual_info::visual_info_decorator;

pub fn get_content_media_panel_query() -> String {
    format!(
        "\
        ... on ContentMediaPanel {{
          variant
          content {{
            {}
          }}
          media {{
            {}
          }}
          visualInfo {{
            textAlignment
            backgroundColor
          }}
        }}
    ",
        get_content_panel_single_reference_query(),
        get_picture_panel_single_reference_query()
    )
}

pub fn content_media_panel_decorator(contentful_content_media_panel: ContentfulContentMediaPanel) -> ContentMediaPanel {
    ContentMediaPanel {
        content: content_panel_decorator(contentful_content_media_panel.content),
        media: picture_panel_decorator(contentful_content_media_panel.media),
        variant: contentful_content_media_panel.variant,
        visual_info: visual_info_decorator(contentful_content_media_panel.visual_info),
    }
}
