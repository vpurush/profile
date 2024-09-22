use crate::contentful_richtext_renderer::types::{ContentfulRichTextDocument, ContentfulRichTextNode};

pub fn render_document_to_html(document: ContentfulRichTextDocument) -> String {
    render_node_list_to_html(document.content)
}

fn render_node_list_to_html(node_list: Vec<ContentfulRichTextNode>) -> String {
    let html_string = node_list.into_iter().map(|content_item| {
        render_node_to_html(content_item)
    }).collect();
    html_string
}

fn render_node_to_html(node: ContentfulRichTextNode) -> String {
    match node {
        ContentfulRichTextNode::Document(document_node) => {
            render_document_to_html(document_node)
        },
        ContentfulRichTextNode::Text(text_node) => {
            format!("{}", text_node.value)
        },
        ContentfulRichTextNode::Paragraph(paragraph_node) => {
            format!("<p>{}</p>", render_node_list_to_html(paragraph_node.content))
        },
        ContentfulRichTextNode::Heading1(heading_node) => {
            format!("<h1>{}</h1>", render_node_list_to_html(heading_node.content))
        },
        ContentfulRichTextNode::Heading2(heading_node) => {
            format!("<h2>{}</h2>", render_node_list_to_html(heading_node.content))
        },
        ContentfulRichTextNode::Heading3(heading_node) => {
            format!("<h3>{}</h3>", render_node_list_to_html(heading_node.content))
        },
        ContentfulRichTextNode::Heading4(heading_node) => {
            format!("<h4>{}</h4>", render_node_list_to_html(heading_node.content))
        },
        ContentfulRichTextNode::Heading5(heading_node) => {
            format!("<h5>{}</h5>", render_node_list_to_html(heading_node.content))
        },
        ContentfulRichTextNode::Heading6(heading_node) => {
            format!("<h6>{}</h6>", render_node_list_to_html(heading_node.content))
        },
        ContentfulRichTextNode::Hyperlink(hyperlink_node) => {
            format!("<a href={}>{}</a>", hyperlink_node.data.uri, render_node_list_to_html(hyperlink_node.content))
        },

    }
}


