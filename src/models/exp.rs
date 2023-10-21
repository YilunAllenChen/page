use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RawExperience {
    pub company: String,
    pub start_time: String,
    pub end_time: String,
    pub location: String,
    pub icon: String,
    pub title: String,
    pub desc: String,
}
