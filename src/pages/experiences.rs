use crate::{
    items::Experience,
    models::{BuiltYaml, OneOfArticle, RawExperience},
    Route,
};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(Experiences)]
pub fn experiences() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let yaml = include_str!("../artifacts/build/compiled.yaml");
    let built_yaml: BuiltYaml = serde_yaml::from_str(yaml).unwrap();

    let mut all_exp: Vec<RawExperience> = built_yaml
        .artifacts
        .into_iter()
        .filter_map(|a| match a {
            OneOfArticle::Experience(exp) => Some(exp),
            _ => None,
        })
        .collect();
    all_exp.sort_by(|a, b| b.start_time.cmp(&a.start_time));

    let all_exp_html: Html = all_exp
        .into_iter()
        .map(|exp| html! {<Experience experience={exp}/>})
        .collect();

    html! {
    <div class="bg-black h-screen">
        <div class="relative isolate px-8 pt-14 lg:px-16" onclick={onclick}>
            <div class="mx-auto max-w-4xl py-8 lg:py-12 bg-opacity-50">
                <ul role="list" class="text-white px-2 md:px-20 lg:px-36 md:py-10 divide-y divide-gray-800"
                     onclick={|e: MouseEvent| e.stop_propagation()}
                >
                    {all_exp_html}
                </ul>
            </div>
        </div>
    </div>
    }
}
