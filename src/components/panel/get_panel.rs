use crate::components::content_media_panel::get_content_media_panel::{content_media_panel_decorator, get_content_media_panel_query};
use crate::components::content_panel::get_content_panel::{
    content_panel_decorator, get_content_panel_query,
};
use crate::components::panel::types::{ContentfulPanel, ContentfulPanelsCollection, Panel};
use crate::components::picture_panel::get_picture_panel::{get_picture_panel_query, picture_panel_decorator};

pub fn get_panel_query() -> String {
    format!(
        "\
        {}
        {}
        {}
    ",
        get_content_panel_query(),
        get_picture_panel_query(),
        get_content_media_panel_query()
    )
}

pub fn get_panels_collection_query() -> String {
    format!(
        "\
        items {{
          typename: __typename
          {}
        }}
    ",
        get_panel_query()
    )
}

pub fn panel_decorator(contentful_panel: ContentfulPanel) -> Panel {
    match contentful_panel {
        ContentfulPanel::ContentPanel(content_panel) => {
            Panel::ContentPanel(content_panel_decorator(content_panel))
        },
        ContentfulPanel::PicturePanel(picture_panel) => {
            Panel::PicturePanel(picture_panel_decorator(picture_panel))
        },
        ContentfulPanel::ContentMediaPanel(content_media_panel) => {
            Panel::ContentMediaPanel(content_media_panel_decorator(content_media_panel))
        }
    }
}
pub fn panels_collection_decorator(
    contentful_panels_collection: ContentfulPanelsCollection,
) -> Vec<Panel> {
    contentful_panels_collection
        .items
        .into_iter()
        .map(|contentful_panel| panel_decorator(contentful_panel))
        .collect()
}
