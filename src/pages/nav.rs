use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

pub struct Nav {
    show: bool,
}

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub on_clicked: Callback<Route>,
}

pub enum Msg {
    ToggleSidebar,
    SelectPage(Route),
}

impl Component for Nav {
    type Message = Msg;
    type Properties = NavProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { show: false }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSidebar => {
                self.show = !self.show;
                true
            }
            Msg::SelectPage(page) => {
                self.show = false;
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&page);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.show {
            html! {
                <div class="z-50 fixed bottom-4 right-4">

                <div class="animate-enter-bottom flex-none rounded-full bg-blue-500/20 p-1.5">
                  <button
                      class="w-12 h-12 bg-blue-500 text-white rounded-full flex items-center justify-center text-2xl"
                      onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                  >
                    {"🧭"}
                  </button>
                </div>
                </div>
            }
        } else {
            let nav_buttons = vec![
                (Route::Home, "cyan"),
                (Route::Contact, "green"),
                (Route::Projects, "indigo"),
                (Route::Experiences, "orange"),
            ]
            .into_iter()
            .map(|(page, color)| {
                let page_name = page.clone().to_string();
                let class = format!(
                    "rounded-md w-2/3 bg-{}-500 my-2.5 px-3.5 py-2.5
                    text-sm font-semibold text-white shadow-sm hover:bg-{}-300",
                    color, color
                );
                html! {
                    <button
                        onclick={ctx.link().callback(move |_|Msg::SelectPage(page.clone()))}
                        class={class}
                    >
                    {page_name}
                    </button>
                }
            });

            html! {
                <div class="relative z-20" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                <div class="fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity"></div>
                <div class="fixed inset-0 z-20 w-screen overflow-y-auto">
                  <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="relative ease-in transform overflow-hidden rounded-lg text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                      <div class="bg-slate-900 px-4 py-5 sm:p-6">
                          <div class="mt-3 text-center sm:ml-4 sm:mt-0">
                            <h3 class="text-base font-semibold leading-6 text-slate-100" id="modal-title">
                              {"Navigate"}
                            </h3>
                            <div class="mt-2 mb-6">
                              <p class="text-sm text-gray-300">
                                {"Navigate to any page easily from here."}
                              </p>
                            </div>
                            {for nav_buttons}
                            <button
                              type="button"
                              class="inline-flex mt-10 w-2/3 justify-center rounded-md bg-red-100 px-3 py-2 text-sm font-semibold text-black shadow-sm hover:bg-red-300"
                              onclick={ctx.link().callback(|_| Msg::ToggleSidebar)}
                            >
                              {"❌"}
                            </button>
                          </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            }
        }
    }
}
