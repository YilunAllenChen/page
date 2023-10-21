use yew::prelude::*;

use crate::models::RawExperience;
use serde::Deserialize;

pub struct Experience {}

#[derive(Properties, PartialEq, Deserialize, Debug)]
pub struct ExperienceProps {
    pub experience: RawExperience,
}

impl Component for Experience {
    type Message = ();
    type Properties = ExperienceProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let exp = &ctx.props().experience;
        html! {
            <>
              <li class="flex justify-between gap-x-6 py-5"
              >
                <img class="h-12 w-12 rounded-full ring ring-slate-800" src={format!("assets/{}", exp.icon)} alt="" />
                <div class="w-full">
                    <div class="flex justify-between min-w-0 gap-x-4">
                        <p class="text-base font-semibold text-gray-100">{exp.company.clone()}</p>
                        <p class="text-base text-gray-300 items-end">{exp.start_time.clone()} {" - "} {exp.end_time.clone()}</p>
                    </div>
                    <div class="flex justify-between min-w-0 mt-2">
                        <p class="text-sm text-gray-400">{exp.title.clone()}</p>
                        <p class="text-sm text-gray-400 items-end">{exp.location.clone()}</p>
                    </div>
                    <div class="mt-4 shrink-0 flex flex-col items-start">
                        <pre class="font-sans text-sm leading-6">
                            {exp.desc.clone()}
                        </pre>
                    </div>
                </div>
              </li>
            </>
        }
    }
}
