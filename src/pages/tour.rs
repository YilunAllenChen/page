use yew::prelude::*;

use crate::models::{MetaYaml, RawArticle};
use crate::projects::Project;
use crate::Page;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuiltYaml {
    pub meta: MetaYaml,
    pub artifacts: Vec<RawArticle>,
}

pub struct Tour;

#[derive(Properties, PartialEq)]
pub struct TourProps {
    pub on_clicked: Callback<Page>,
}

impl Component for Tour {
    type Message = ();
    type Properties = TourProps;

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
        let yaml = include_str!("../artifacts/build/compiled_projects.yaml");
        let built_yaml: BuiltYaml = serde_yaml::from_str(yaml).unwrap();

        let mut article_props = built_yaml.artifacts;
        article_props.sort_by(|a, b| a.time.cmp(&b.time));
        let articles: Html = article_props
            .into_iter()
            .rev()
            .map(|proj| {
                html! {
                    <Project project={proj}/>
                }
            })
            .collect();

        html! {
        <div class="mx-auto mt-10 grid px-8 grid-cols-1 gap-x-8 gap-y-16 pt-10 lg:grid-cols-2">
            {articles}
        </div>
        }
    }
}
