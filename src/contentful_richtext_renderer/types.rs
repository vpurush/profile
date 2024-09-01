use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulRichTextDocument {
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulRichTextContent {
    json: ContentfulRichTextDocument,
}