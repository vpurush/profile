use crate::components::content_panel::get_content_panel::{content_panel_decorator, get_content_panel_query};
use crate::components::panel::types::{ContentfulPanel, ContentfulPanelsCollection, Panel};

pub fn get_panel_query() -> String {
    format!("\
        {}
    ", get_content_panel_query())
}

pub fn get_panels_collection_query() -> String {
    format!("\
        items {{
          typename: __typename
          {}
        }}
    ", get_panel_query())
}

pub fn panel_decorator(contentful_panel: ContentfulPanel) -> Panel {
    match contentful_panel {
        ContentfulPanel::ContentPanel(content_panel) => {
            Panel::ContentPanel(content_panel_decorator(content_panel))
        }
    }
}
pub fn panels_collection_decorator(contentful_panels_collection: ContentfulPanelsCollection) -> Vec<Panel> {
    contentful_panels_collection.items.into_iter().map(|contentful_panel| panel_decorator(contentful_panel)).collect()
}