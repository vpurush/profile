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