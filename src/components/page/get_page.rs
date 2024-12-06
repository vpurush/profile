use crate::common::application_error::ApplicationError;
use crate::common::query_contentful::{query_contentful, ContentfulQueryPostData};
use crate::common::types::{ContentfulResponse, ContentfulResponseItems};
use crate::components::page::types::{ContentfulPage, ContentfulPageCollection, Page, PageCollection};
use crate::components::panel::get_panel::{get_panel_query, get_panels_collection_query, panels_collection_decorator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct PostVariables {
    slug: String,
}

pub fn get_page_query() -> String {
    format!(
        r#"
        query($slug: String!) {{
          pageCollection (preview: false, where: {{slug: $slug}}, limit: 1) {{
            items {{
              title
              slug
              panelsCollection (limit: 20) {{
                {}
              }}
            }}
          }}
        }}"#,
        get_panels_collection_query()
    )
}

pub fn page_decorator (contentful_page: ContentfulPage) -> Page {
    Page {
        title: contentful_page.title,
        slug: contentful_page.slug,
        panels_collection: panels_collection_decorator(contentful_page.panels_collection)
    }
}
pub fn page_collection_decorator (contentful_page_collection: ContentfulPageCollection) -> PageCollection {
    PageCollection {
        page_collection: contentful_page_collection.page_collection.items.into_iter().map(|contentful_page| page_decorator(contentful_page)).collect()
    }
}
pub async fn get_page_collection(slug: String) -> Result<PageCollection, ApplicationError> {
    println!("query {}", get_page_query());
    let post_data = ContentfulQueryPostData {
        query: get_page_query(),
        variables: PostVariables { slug },
    };

    let page_collection = query_contentful::<ContentfulPageCollection, PostVariables>(post_data).await?;

    let decorated_page_collection = page_collection_decorator(page_collection);

    Ok(decorated_page_collection)
}
