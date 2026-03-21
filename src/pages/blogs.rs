use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Link};

use crate::{
    items::Blog,
    models::{BuiltYaml, OneOfArticle, RawBlog},
    Route,
};

#[derive(Properties, PartialEq)]
pub struct BlogsProps {
    #[prop_or_default]
    pub slug: Option<String>,
}

fn load_blogs() -> Vec<RawBlog> {
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
    all_blogs
}

fn excerpt(body: &str) -> String {
    let collapsed = body
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .find(|line| !line.starts_with('#') && !line.starts_with("```") && !line.starts_with("!["))
        .unwrap_or(body)
        .replace("**", "")
        .replace('*', "")
        .replace('`', "");

    let mut chars = collapsed.chars();
    let preview: String = chars.by_ref().take(160).collect();
    if chars.next().is_some() {
        format!("{preview}...")
    } else {
        preview
    }
}

#[function_component(Blogs)]
pub fn my_component(props: &BlogsProps) -> Html {
    let all_blogs = load_blogs();
    let navigator = use_navigator().unwrap();

    if let Some(slug) = &props.slug {
        let current_index = all_blogs.iter().position(|blog| blog.slug == *slug);

        return match current_index {
            Some(index) => {
                let blog = all_blogs[index].clone();
                let prev_route = index.checked_sub(1).map(|prev_index| Route::BlogPost {
                    slug: all_blogs[prev_index].slug.clone(),
                });
                let next_route = (index + 1 < all_blogs.len()).then(|| Route::BlogPost {
                    slug: all_blogs[index + 1].slug.clone(),
                });

                let prev_onclick = {
                    let navigator = navigator.clone();
                    let prev_route = prev_route.clone();
                    Callback::from(move |_| {
                        if let Some(route) = &prev_route {
                            navigator.push(route);
                        }
                    })
                };
                let next_onclick = {
                    let navigator = navigator.clone();
                    let next_route = next_route.clone();
                    Callback::from(move |_| {
                        if let Some(route) = &next_route {
                            navigator.push(route);
                        }
                    })
                };

                let floating_button_class = |enabled: bool| {
                    if enabled {
                        "h-12 rounded-full bg-blue-500 px-4 text-sm font-semibold text-white transition hover:bg-blue-400"
                    } else {
                        "h-12 rounded-full bg-slate-800 px-4 text-sm font-semibold text-slate-500 cursor-not-allowed"
                    }
                };

                html! {
                    <>
                        <div class="mx-auto flex min-h-screen max-w-4xl flex-col gap-6 px-4 py-10 md:px-6">
                            <Blog blog={blog}/>
                        </div>
                        <div class="fixed bottom-4 right-20 z-40">
                            <div class="animate-enter-bottom flex items-center gap-2 rounded-full bg-blue-500/20 p-1.5">
                                <button
                                    class={floating_button_class(prev_route.is_some())}
                                    onclick={prev_onclick}
                                    disabled={prev_route.is_none()}
                                    aria-label="Go to previous blog"
                                    title="Go back"
                                >
                                    {"⬅️"}
                                </button>
                                <Link<Route>
                                    to={Route::Blogs}
                                    classes="inline-flex h-12 items-center justify-center rounded-full bg-blue-500 px-4 text-sm font-semibold text-white transition hover:bg-blue-400"
                                >
                                    {"📚"}
                                </Link<Route>>
                                <button
                                    class={floating_button_class(next_route.is_some())}
                                    onclick={next_onclick}
                                    disabled={next_route.is_none()}
                                    aria-label="Go to next blog"
                                    title="Go next"
                                >
                                    {"➡️"}
                                </button>
                            </div>
                        </div>
                    </>
                }
            }
            None => html! {
                <div class="mx-auto flex min-h-screen max-w-3xl flex-col items-start justify-center gap-4 px-4 py-10 text-slate-200 md:px-6">
                    <h1 class="text-3xl font-semibold text-slate-100">{"Blog not found"}</h1>
                    <p class="text-slate-400">{"That post does not exist in the compiled blog artifacts."}</p>
                    <Link<Route>
                        to={Route::Blogs}
                        classes="inline-flex items-center rounded-full border border-slate-700 px-4 py-2 text-sm text-slate-300 transition hover:border-sky-400 hover:text-sky-300"
                    >
                        {"Back to all blogs"}
                    </Link<Route>>
                </div>
            },
        };
    }

    let cards: Html = all_blogs
        .into_iter()
        .map(|blog| {
            let route = Route::BlogPost {
                slug: blog.slug.clone(),
            };
            let preview = excerpt(&blog.body);

            html! {
                <Link<Route> to={route} classes="block rounded-2xl border border-slate-800 bg-slate-950/90 p-6 text-slate-100 shadow-xl shadow-slate-950/30 transition hover:-translate-y-1 hover:border-sky-500/60 hover:shadow-sky-950/20">
                    <div class="flex items-start justify-between gap-4">
                        <div>
                            <h2 class="text-2xl font-semibold tracking-tight">{blog.title}</h2>
                        </div>
                        <p class="shrink-0 pt-1 text-sm text-slate-400">{blog.time}</p>
                    </div>
                    <div class="mt-4 flex flex-wrap gap-2">
                        {for blog.tags.iter().map(|tag| html! {
                            <span class="inline-flex items-center rounded-full border border-sky-500/40 px-2.5 py-1 text-xs font-medium text-sky-200">
                                {tag}
                            </span>
                        })}
                    </div>
                    <p class="mt-6 text-base leading-7 text-slate-300">{preview}</p>
                </Link<Route>>
            }
        })
        .collect();

    html! {
        <div class="mx-auto flex min-h-screen max-w-5xl flex-col gap-8 px-4 py-10 md:px-6">
            <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                {cards}
            </div>
        </div>
    }
}
