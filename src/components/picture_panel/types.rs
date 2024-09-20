use serde::{Deserialize, Serialize};
use crate::components::visual_info::types::{ContentfulVisualInfo, VisualInfo};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulAsset {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPicturePanel {
    pub title: Option<String>,
    #[serde(rename(serialize = "image", deserialize = "image"))]
    pub image: ContentfulAsset,
    #[serde(rename(serialize = "imageCaption", deserialize = "imageCaption"))]
    pub image_caption: String,
    #[serde(rename(serialize = "visualInfo", deserialize = "visualInfo"))]
    pub visual_info: Option<ContentfulVisualInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub url: String,
    pub width: u32,
    pub height: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PicturePanel {
    pub title: Option<String>,
    pub image: Asset,
    pub image_caption: String,
    pub visual_info: VisualInfo
}