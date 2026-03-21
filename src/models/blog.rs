use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RawBlog {
    #[serde(default)]
    pub slug: String,
    pub time: String,
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
}
