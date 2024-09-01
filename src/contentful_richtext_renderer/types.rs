use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum ContentfulRichTextNode {
    #[serde(rename(serialize = "document", deserialize = "document"))]
    Document(ContentfulRichTextDocument),
    #[serde(rename(serialize = "paragraph", deserialize = "paragraph"))]
    Paragraph(ContentfulRichTextParagraph),
    #[serde(rename(serialize = "text", deserialize = "text"))]
    Text(ContentfulRichTextText),
    #[serde(rename(serialize = "heading-1", deserialize = "heading-1"))]
    Heading1(ContentfulRichTextHeading1),
    #[serde(rename(serialize = "heading-2", deserialize = "heading-2"))]
    Heading2(ContentfulRichTextHeading2),
    #[serde(rename(serialize = "heading-3", deserialize = "heading-3"))]
    Heading3(ContentfulRichTextHeading3),
    #[serde(rename(serialize = "heading-4", deserialize = "heading-4"))]
    Heading4(ContentfulRichTextHeading4),
    #[serde(rename(serialize = "heading-5", deserialize = "heading-5"))]
    Heading5(ContentfulRichTextHeading5),
    #[serde(rename(serialize = "heading-6", deserialize = "heading-6"))]
    Heading6(ContentfulRichTextHeading6),
    #[serde(rename(serialize = "hyperlink", deserialize = "hyperlink"))]
    Hyperlink(ContentfulRichTextHyperlink)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextDocument {
    pub content: Vec<ContentfulRichTextNode>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextParagraph {
    pub content: Vec<ContentfulRichTextNode>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextText {
    pub value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHeading1 {
    pub content: Vec<ContentfulRichTextNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHeading2 {
    pub content: Vec<ContentfulRichTextNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHeading3 {
    pub content: Vec<ContentfulRichTextNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHeading4 {
    pub content: Vec<ContentfulRichTextNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHeading5 {
    pub content: Vec<ContentfulRichTextNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHeading6 {
    pub content: Vec<ContentfulRichTextNode>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHyperlink {
    pub data: ContentfulRichTextHyperlinkData,
    pub content: Vec<ContentfulRichTextNode>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub struct ContentfulRichTextHyperlinkData {
    pub uri: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulRichTextContent {
    pub json: ContentfulRichTextDocument,
}