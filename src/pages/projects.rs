use yew::prelude::*;

use crate::items::Project;
use crate::models::{BuiltYaml, OneOfArticle, RawProject};

pub struct Projects;

#[derive(Properties, PartialEq)]
pub struct Props {}

impl Component for Projects {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let yaml = include_str!("../artifacts/build/compiled.yaml");
        let built_yaml: BuiltYaml = serde_yaml::from_str(yaml).unwrap();

        let mut all_artifacts: Vec<RawProject> = built_yaml
            .artifacts
            .into_iter()
            .filter_map(|a| match a {
                OneOfArticle::Project(proj) => Some(proj),
                _ => None,
            })
            .collect();
        all_artifacts.sort_by(|a, b| a.time.cmp(&b.time));
        let articles: Html = all_artifacts
            .into_iter()
            .rev()
            .map(|proj| {
                html! {
                    <Project project={proj}/>
                }
            })
            .collect();

        html! {
        <div class="bg-black mx-auto mt-10 grid px-8 grid-cols-1 gap-x-8 gap-y-16 pt-10 lg:grid-cols-2 md:mx-16 md:gap-x-16 xl:px-4">
            {articles}
        </div>
        }
    }
}
