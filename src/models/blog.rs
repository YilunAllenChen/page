use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RawBlog {
    pub time: String,
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
}
