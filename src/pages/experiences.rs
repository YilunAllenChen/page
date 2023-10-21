use yew::prelude::*;

use crate::{
    items::Experience,
    models::{BuiltYaml, OneOfArticle, RawExperience},
    Page,
};

pub struct About;

#[derive(Properties, PartialEq)]
pub struct AboutProps {
    pub on_clicked: Callback<Page>,
}

impl Component for About {
    type Message = ();
    type Properties = AboutProps;

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
        <div class="bg-black h-full">
            <div class="relative isolate px-2 pt-14 lg:px-8">
                <div class="mx-auto max-w-4xl py-8 lg:py-12 bg-opacity-50">
                    <ul role="list" class="text-white px-2 md:px-20 lg:px-36 md:py-10 divide-y divide-gray-800">
                        {all_exp_html}
                    </ul>
                </div>
            </div>
        </div>
        }
    }
}
