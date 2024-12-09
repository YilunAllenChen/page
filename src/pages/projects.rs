use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::items::Project;
use crate::models::{BuiltYaml, OneOfArticle, RawProject};
use crate::Route;

#[function_component(Projects)]
pub fn my_component() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

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
    <div class=r#"
            bg-black mx-auto mt-10 grid px-8 grid-cols-1 gap-x-8 gap-y-16 pt-10 
            md:mx-16 md:gap-x-32 md:grid-cols-2 
            lg:gap-x-32 lg:px-32
            xl:px-4
    "#
         onclick={onclick}>
        {articles}
    </div>
    }
}
