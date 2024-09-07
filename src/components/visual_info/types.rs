use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentfulVisualInfoTextAlign {
    Left,
    Right,
    Center,
    Justified
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulVisualInfo {
    #[serde(rename(serialize = "textAlignment", deserialize = "textAlignment"))]
    pub text_alignment: Vec<ContentfulVisualInfoTextAlign>,
    #[serde(rename(serialize = "backgroundColor", deserialize = "backgroundColor"))]
    pub background_color: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualInfo {
    pub text_alignment: ContentfulVisualInfoTextAlign,
    pub background_color: String,
}
