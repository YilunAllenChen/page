use yew::prelude::*;

use crate::{
    html_utils::{make_tag, render_markdown_to_html},
    models::RawBlog,
};
use serde::Deserialize;

#[derive(Properties, PartialEq, Deserialize, Debug)]
pub struct BlogProps {
    pub blog: RawBlog,
}

#[function_component(Blog)]
pub fn experiences(BlogProps { blog }: &BlogProps) -> Html {
    let tags: Html = blog
        .tags
        .iter()
        .map(|tag| Html::from_html_unchecked(make_tag(tag, "sky").into()))
        .collect();

    html! {
        <article class="w-full rounded-2xl border border-slate-800 bg-slate-950/90 p-6 text-white shadow-2xl shadow-slate-950/50 md:p-8">
            <div class="mb-6 flex flex-col gap-4 border-b border-slate-800 pb-6">
                <div class="flex flex-col gap-2 md:flex-row md:items-end md:justify-between">
                    <div>
                        <h1 class="text-3xl font-semibold tracking-tight text-slate-100 md:text-4xl">
                            {blog.title.clone()}
                        </h1>
                    </div>
                    <p class="text-sm text-slate-400">{blog.time.clone()}</p>
                </div>
                <div class="flex flex-wrap gap-2">
                    {tags}
                </div>
            </div>
            <div class="blog max-w-none">
                {Html::from_html_unchecked(render_markdown_to_html(&blog.body).into())}
            </div>
        </article>
    }
}
