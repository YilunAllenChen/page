use yew::prelude::*;

use crate::models::RawExperience;
use serde::Deserialize;

#[derive(Properties, PartialEq, Deserialize, Debug)]
pub struct ExperienceProps {
    pub experience: RawExperience,
}

#[function_component(Experience)]
pub fn experiences(ExperienceProps { experience }: &ExperienceProps) -> Html {
    let exp = &experience;
    html! {
        <>
          <li class="flex justify-between gap-x-4 py-5" >
            <div class="w-full">
                <div class="flex justify-left min-w-0 gap-x-2 md:gap-x-4">
                    <img class="h-12 w-12 rounded-full ring ring-slate-800" src={format!("assets/{}", exp.icon)} alt="" />
                    <div class="flex justify-between items-center w-full min-w-0 gap-x-4">
                        <div>
                            <p class="text-sm md:text-base font-semibold text-gray-100">{exp.company.clone()}</p>
                            <p class="text-xs md:text-sm text-gray-400">{exp.title.clone()}</p>
                        </div>
                        <div class="justify-end">
                            <p class="text-xs md:text-sm text-gray-300 items-end text-right">{exp.start_time.clone()} {" - "} {exp.end_time.clone()}</p>
                            <p class="text-xs md:text-sm text-gray-400 items-end text-right">{exp.location.clone()}</p>
                        </div>
                    </div>
                </div>
                <div class="mt-4 items-start">
                    {Html::from_html_unchecked(exp.desc.clone().into())}
                </div>
            </div>
          </li>
        </>
    }
}
