use crate::Page;
use yew::prelude::*;

pub struct Home;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub on_clicked: Callback<Page>,
}

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
        html! {
            <div class="bg-black h-full bg-cover bg-center" style="background-image: url('./assets/robot_and_seagull.png')">
                <div class="relative isolate px-6 pt-14 lg:px-8">
                    <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56 bg-opacity-50">
                        <div class="text-center">
                            <h1 class="text-4xl font-bold tracking-wider text-gray-100 sm:text-6xl font-mono">{"Allen Chen"}</h1>
                            <p class="mt-6 text-lg leading-8 text-gray-300">{"I Built Stuff."}</p>
                            <div class="mt-10 flex flex-wrap items-center justify-center gap-x-6 mb-20">
                                <button
                                    onclick={ctx.props().on_clicked.reform(|_| Page::Projects)}
                                    class="rounded-md w-full md:w-1/4 bg-indigo-600 my-2.5 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                    {"Projects"}
                                </button>
                                <button
                                    onclick={ctx.props().on_clicked.reform(|_| Page::Contact)}
                                    class="rounded-md w-full md:w-1/4 bg-green-600 my-2.5 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-green-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                    {"Contact Me"}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        }
    }
}
