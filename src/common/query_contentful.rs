use std::error::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::common::application_error::ApplicationError;
use crate::common::types::AEMResponse;


#[derive(Debug, Serialize, Deserialize)]
pub struct ContentfulQueryPostData<T: Serialize> {
    pub query: String,
    pub variables: T,
}

pub async fn query_contentful<ReturnType: DeserializeOwned, QueryVariableType: Serialize>(post_data: ContentfulQueryPostData<QueryVariableType>) -> Result<ReturnType, ApplicationError>
{
    let contentful_url = crate::common::constants::get_constants()?.CONTENTFUL_DELIVERY_URL;
    let contentful_token = crate::common::constants::get_constants()?.CONTENTFUL_PREVIEW_TOKEN;

    println!("contentful_url {} contentful_token {}", contentful_url, contentful_token);

    let contentful_call = reqwest::Client::new()
        .post(contentful_url)
        .header("Authorization", format!("Bearer {}", contentful_token))
        .json(&post_data)
        .send()
        .await?;

    match contentful_call.error_for_status_ref() {
        Ok(_) => Ok(contentful_call.json::<AEMResponse<ReturnType>>()
            .await?.data),
        Err(error) => {
            println!("Contentful call failed {}", contentful_call.text().await?);
            Err(ApplicationError::ReqwestError(error))
        }
    }
}