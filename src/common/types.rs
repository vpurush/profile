use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentfulResponseItems<T> {
    pub items: Vec<T>,
}
