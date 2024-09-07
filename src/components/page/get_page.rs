use crate::common::application_error::ApplicationError;
use crate::common::query_contentful::{query_contentful, ContentfulQueryPostData};
use crate::common::types::{ContentfulResponse, ContentfulResponseItems};
use crate::components::page::types::ContentfulPageCollection;
use crate::components::panel::get_panel::{get_panel_query, get_panels_collection_query};
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
          pageCollection (preview: true, where: {{slug: $slug}}) {{
            items {{
              title
              slug
              panelsCollection {{
                {}
              }}
            }}
          }}
        }}"#,
        get_panels_collection_query()
    )
}
pub async fn get_page(slug: String) -> Result<ContentfulPageCollection, ApplicationError> {
    println!("query {}", get_page_query());
    let postData = ContentfulQueryPostData {
        query: get_page_query(),
        variables: PostVariables { slug },
    };

    let page_collection = query_contentful::<ContentfulPageCollection, PostVariables>(postData).await?;

    // let response = match contentful_json {
    //     Ok(response) => Ok(response),
    //     Err(error) => {
    //         println!("Error occurred in get_page {}", error);
    //         Err(error.into())
    //     }
    // };
    println!("response {:?}", page_collection);
    Ok(page_collection)

    // Ok(AEMResponse {
    //     data: PageCollection {
    //         page_collection: AEMResponseItems {
    //             items: [
    //                 ContentfulPage {
    //                     slug: "s".to_string(),
    //                     title: "t".to_string(),
    //                 }
    //             ]
    //         }
    //     }
    // })
}
