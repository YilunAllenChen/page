use yew::prelude::*;

use crate::{html_utils::render_markdown_to_html, models::RawBlog};
use serde::Deserialize;

#[derive(Properties, PartialEq, Deserialize, Debug)]
pub struct BlogProps {
    pub blog: RawBlog,
}

#[function_component(Blog)]
pub fn experiences(BlogProps { blog }: &BlogProps) -> Html {
    let exp = &blog;
    html! {
        <>
          <div class="text-white w-full" >
            <div class="blog p-8 m-8 rounded-sm bg-slate-900 ">
            { Html::from_html_unchecked(render_markdown_to_html(&exp.body).into()) }
            </div>
          </div>
        </>
    }
}
