use std::env;
use std::env::VarError;
use std::error::Error;
use std::string::ToString;

// pub static CONTENTFUL_DELIVERY_URL: &str = "https://graphql.contentful.com/content/v1/spaces/lb6xmef9cbrw/environments/master";
// pub static CONTENTFUL_PREVIEW_TOKEN: Result<String, VarError> = env::var("CONTENTFUL_PREVIEW_TOKEN");

pub struct Constants<'a> {
    pub CONTENTFUL_DELIVERY_URL: &'a str,
    pub CONTENTFUL_PREVIEW_TOKEN: String,
}
pub fn get_constants<'a>() -> Result<Constants<'a>, VarError> {
    Ok(Constants{
        CONTENTFUL_DELIVERY_URL: "https://graphql.contentful.com/content/v1/spaces/lb6xmef9cbrw/environments/master",
        CONTENTFUL_PREVIEW_TOKEN: env::var("CONTENTFUL_PREVIEW_TOKEN")?,
    })
}