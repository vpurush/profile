use crate::common::types::{AEMResponse, AEMResponseItems};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

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
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    query: String,
    variables: PostVariables,
}

pub async fn get_page(slug: String) -> Result<AEMResponse<PageCollection>, Box<dyn Error>> {
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

    let postData = Post {
        query: page_query.to_string(),
        variables: PostVariables { slug },
    };

    let contentful_url = crate::common::constants::get_constants()?.CONTENTFUL_DELIVERY_URL;
    let contentful_token = crate::common::constants::get_constants()?.CONTENTFUL_PREVIEW_TOKEN;

    println!("contentful_url {} contentful_token {}", contentful_url, contentful_token);

    let contentful_call = reqwest::Client::new()
        .post(contentful_url)
        .header("Authorization", format!("Bearer {}", contentful_token))
        .json(&postData)
        .send()
        .await.unwrap();

    let contentful_json = match contentful_call.error_for_status_ref() {
        Ok(_) => contentful_call.json::<AEMResponse<PageCollection>>()
            .await,
        Err(error) => {
            println!("Contentful call failed {}", contentful_call.text().await.unwrap());
            Err(error)
        }
    };


    let response = match contentful_json {
        Ok(response) => Ok(response),
        Err(error) => {
            println!("Error occurred in get_page {}", error);
            Err(error.into())
        }
    };
    println!("response {:?}", response);
    response

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
