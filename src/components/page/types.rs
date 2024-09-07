use serde::{Deserialize, Serialize};
use crate::common::types::ContentfulResponseItems;
use crate::components::panel::types::ContentfulPanelsCollection;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPage {
    pub title: String,
    pub slug: String,
    #[serde(rename(serialize = "panelsCollection", deserialize = "panelsCollection"))]
    pub panels_collection: ContentfulPanelsCollection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPageCollection {
    #[serde(rename(serialize = "pageCollection", deserialize = "pageCollection"))]
    pub page_collection: ContentfulResponseItems<ContentfulPage>,
}
