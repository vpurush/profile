use serde::{Deserialize, Serialize};
use crate::common::types::ContentfulResponseItems;
use crate::components::panel::types::{ContentfulPanelsCollection, Panel};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub title: String,
    pub slug: String,
    pub panels_collection: Vec<Panel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageCollection {
    pub page_collection: Vec<Page>,
}