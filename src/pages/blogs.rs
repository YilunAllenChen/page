use yew::prelude::*;

use crate::{
    items::Blog,
    models::{BuiltYaml, OneOfArticle, RawBlog},
    Route,
};
use yew_router::hooks::use_navigator;

#[function_component(Blogs)]
pub fn my_component() -> Html {
    // let navigator = use_navigator().unwrap();
    // let onclick = Callback::from(move |_: MouseEvent| navigator.push(&Route::Home));

    let yaml = include_str!("../artifacts/build/compiled.yaml");
    let built_yaml: BuiltYaml = serde_yaml::from_str(yaml).unwrap();

    let mut all_blogs: Vec<RawBlog> = built_yaml
        .artifacts
        .into_iter()
        .filter_map(|a| match a {
            OneOfArticle::Blog(blog) => Some(blog),
            _ => None,
        })
        .collect();

    all_blogs.sort_by(|a, b| b.time.cmp(&a.time));

    let all_blogs_html: Html = all_blogs
        .into_iter()
        .map(|blog| html! {<Blog blog={blog}/>})
        .collect();

    html! {
        <>
            {all_blogs_html}
        </>
    }
}
