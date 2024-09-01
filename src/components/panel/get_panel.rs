use serde::{Deserialize, Serialize};

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