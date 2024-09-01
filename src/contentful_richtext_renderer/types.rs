use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum ContentfulRichTextNode {
    #[serde(rename(serialize = "document", deserialize = "document"))]
    Document(ContentfulRichTextDocument),
    #[serde(rename(serialize = "paragraph", deserialize = "paragraph"))]
    Paragraph(ContentfulRichTextParagraph),
    #[serde(rename(serialize = "text", deserialize = "text"))]
    Text(ContentfulRichTextText)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextDocument {
    content: Vec<ContentfulRichTextNode>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextParagraph {
    content: Vec<ContentfulRichTextNode>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextText {
    value: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulRichTextContent {
    json: ContentfulRichTextDocument,
}