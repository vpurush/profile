use serde::{Deserialize, Serialize};
use crate::common::types::AEMResponseItems;
use crate::components::panel::types::PanelsCollection;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPage {
    pub title: String,
    pub slug: String,
    #[serde(rename(serialize = "panelsCollection", deserialize = "panelsCollection"))]
    pub panels_collection: PanelsCollection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageCollection {
    #[serde(rename(serialize = "pageCollection", deserialize = "pageCollection"))]
    pub page_collection: AEMResponseItems<ContentfulPage>,
}
