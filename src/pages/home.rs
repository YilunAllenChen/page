use crate::Route;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

pub struct Home;

#[derive(Properties, PartialEq)]
pub struct HomeProps {}

impl Component for Home {
    type Message = ();
    type Properties = HomeProps;

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
        let navigator = ctx.link().navigator().unwrap();
        let nav = |page| Callback::from(move |_| navigator.push(page));
        html! {
            <div class="bg-black h-screen bg-cover bg-center bg-[url('./assets/background_mobile.png')] md:bg-[url('./assets/background.png')]" >
                <div class="absolute inset-0 bg-black bg-opacity-70"></div>
                <div class="relative px-6 lg:px-8">
                    <div class="mx-auto max-w-2xl h-screen py-[30vh] lg:scale-[1.2] xl:scale-[1.5]">
                        <div class="text-center">
                            <h1 class="text-2xl md:text-4xl tracking-wider text-gray-100 sm:text-6xl font-mono">{r#"Yilun "Allen" Chen"#}</h1>
                            <div class="mt-10 flex flex-wrap items-center justify-center gap-x-6 gap-y-1 mb-10">
                                <button
                                    onclick={nav.clone()(&Route::Projects)}
                                    class="rounded-md w-2/3 md:w-1/4 bg-indigo-600 my-2.5 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                    {"Projects"}
                                </button>
                                <button
                                    onclick={nav.clone()(&Route::Experiences)}
                                    class="rounded-md w-2/3 md:w-1/4 bg-yellow-600 my-2.5 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-yellow-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orange-600">
                                    {"Experiences"}
                                </button>
                                <button
                                    onclick={nav.clone()(&Route::Contact)}
                                    class="rounded-md w-2/3 md:w-1/4 bg-green-600 my-2.5 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-green-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                    {"Contact"}
                                </button>
                            </div>
                            <p class="mt-2 text-xs md:text-sm text-bold text-gray-200">{"Built with Rust/Yew, Tailwind CSS and ❤️"}</p>
                            <p class="mt-8 text-xs md:text-xs text-bold text-gray-400">{"YilunAllenChen @ 2024, All Rights Reserved"}</p>
                        </div>
                    </div>
                </div>
            </div>

        }
    }
}
