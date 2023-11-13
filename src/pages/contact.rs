use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

pub struct Contact;

#[derive(Properties, PartialEq)]
pub struct ContactProps {}

impl Component for Contact {
    type Message = ();
    type Properties = ContactProps;

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
        let nav = |page| Callback::from(move |_| navigator.push(&page));

        html! {
            <div class="bg-black h-screen">
                <div class="px-6 lg:px-8 h-screen flex flex-col justify-center">
                    <div class="mx-auto my-auto max-w-2xl p-12 sm:my-48 lg:my-56 bg-opacity-50 rounded-lg bg-slate-800">
                        <div class="text-center">
                            <h1 class="text-xl font-bold tracking-tight text-gray-100 sm:text-4xl">{"Let's get in touch!"}</h1>
                            <div class="mt-10 text-center">
                                <div class="w-full rounded-md mb-2 bg-yellow-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-yellow-500">
                                    <a href="mailto:allenchenyilun1999@gmail.com" >{"Email Me"}</a>
                                </div>
                                <p class="text-gray-100 text-sm">{"allenchenyilun1999@gmail.com"}</p>
                            </div>
                            <button
                                onclick={nav(Route::Home)}
                                class="rounded-md w-full mt-20 bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500">
                                {"Home"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
