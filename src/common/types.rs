use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AEMResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AEMResponseItems<T> {
    pub items: [T;1],
}
