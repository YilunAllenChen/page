mod exp;
pub use exp::RawExperience;

mod proj;
pub use proj::{Language, ProjectStatus, RawProject};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum OneOfArticle {
    Experience(RawExperience),
    Project(RawProject),
}

#[derive(Deserialize, Debug, Serialize)]
pub struct BuiltYaml {
    pub artifacts: Vec<OneOfArticle>,
    pub meta: MetaYaml,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MetaYaml {
    pub build: String,
}
