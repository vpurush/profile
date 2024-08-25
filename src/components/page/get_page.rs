use crate::common::types::{AEMResponse, AEMResponseItems};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use crate::common::application_error::ApplicationError;
use crate::common::query_contentful::{query_contentful, ContentfulQueryPostData};

// #[derive(Serialize, Deserialize, Debug)]
// struct PanelsCollection {
//     items: [HashMap<String, String>],
// }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulPage {
    title: String,
    slug: String,
    // panels_collection: PanelsCollection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageCollection {

    #[serde(rename(serialize = "pageCollection", deserialize = "pageCollection"))]
    pub page_collection: AEMResponseItems<ContentfulPage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostVariables {
    slug: String,
}
pub async fn get_page(slug: String) -> Result<PageCollection, ApplicationError> {
    let page_query = r#"
query($slug: String!) {
  pageCollection (preview: true, where: {slug: $slug}) {
    items {
      title
      slug
      panelsCollection {
        items {
          __typename
        }
      }
    }
  }
}
"#;

    let postData = ContentfulQueryPostData {
        query: page_query.to_string(),
        variables: PostVariables { slug },
    };


    let page_collection = query_contentful::<PageCollection, PostVariables>(postData).await?;


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
