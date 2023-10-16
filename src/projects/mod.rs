use yew::prelude::*;

use crate::html_utils::make_tag;

use crate::models::{ProjectStatus, RawArticle};
use serde::Deserialize;

pub struct Project {}

#[derive(Properties, PartialEq, Deserialize, Debug)]
pub struct ProjectProps {
    pub project: RawArticle,
}

impl Component for Project {
    type Message = ();
    type Properties = ProjectProps;

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
        let proj = &ctx.props().project;

        let (text, color) = match proj.status {
            ProjectStatus::Completed => ("Completed", "green"),
            ProjectStatus::Ongoing => ("Ongoing", "yellow"),
            ProjectStatus::Discontinued => ("Discontinued", "red"),
        };

        let dot_class = format!("flex-none rounded-full bg-{}-500/20 p-1.5", color);
        let dot_inner_class = format!("h-2 w-2 rounded-full bg-{}-500", color);
        let dot_and_text = html! {
            <div class="flex items-center gap-x-1.5">
                <p class="text-base leading-5 text-slate-200">{text}</p>
                <div class={dot_class}>
                    <div class={dot_inner_class}></div>
                </div>
            </div>
        };

        let tags: Html = proj
            .tags
            .iter()
            .map(|tag| {
                Html::from_html_unchecked(
                    make_tag(
                        tag,
                        match tag.as_str() {
                            "Music" | "Art" | "Graphics" => "green",
                            "Startup" => "fuchsia",
                            "Trading" | "Finance" => "cyan",
                            "Robotics" | "Hardware" | "IoT" => "blue",
                            "Cloud" => "red",
                            "Game" => "purple",
                            _ => "gray",
                        },
                    )
                    .into(),
                )
            })
            .collect();

        let language_tags = proj
            .languages
            .iter()
            .map(|lang| Html::from_html_unchecked(lang.to_tag().into()))
            .collect::<Vec<Html>>();

        html! {
          <article class="bg-slate-900 flex rounded-lg p-4 md:p-8 flex-col items-start justify-between">
            <div class="flex w-full justify-between gap-x-2 text-xs">
            <div>
                <time datetime="2020-03-16" class="text-gray-300 text-base">{proj.time.clone()}</time>
            </div>
            <div>
                {dot_and_text}
            </div>
            </div>
            <div class="group relative">
            <h3 class="mt-3 text-3xl font-semibold leading-6 text-gray-100 group-hover:text-blue-400">
                <a href={proj.link.clone()} target="_blank">
                <span class="text-xl"></span>
                {proj.title.clone()}
                </a>
            </h3>
            <div class="flex pt-4 items-center gap-x-2 text-xs">
              {language_tags}
              {tags}
            </div>
            <pre class="mt-5 font-sans line-clamp-10 text-base leading-6 text-gray-300">{proj.desc.clone()}</pre>
            </div>
            <div class="w-full pt-4">
              <img src={proj.preview.clone()} class="object-cover rounded-lg h-72 w-full ..."/>
            </div>
        </article>
        }
    }
}
