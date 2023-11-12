use std::fmt::Display;

use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

mod code;
mod html_utils;
mod items;
mod models;
mod pages;

use pages::{Contact, Experiences, Home, Nav, Projects, Wip};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    Projects,
    #[at("/experiences")]
    Experiences,
    #[at("/*_path")]
    Wip { _path: String },
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Route::Home => "Home",
            Route::Contact => "Contact",
            Route::Projects => "Projects",
            Route::Experiences => "Experiences",
            _ => "WIP",
        };
        write!(f, "{}", name)
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Contact => html! {<Contact/>},
        Route::Experiences => html! {<Experiences />},
        Route::Projects => html! {<Projects />},
        Route::Wip { _path } => html! {<Wip />},
    }
}

pub struct App {
    active_page: Route,
}

pub enum Msg {
    GoToPage(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active_page: Route::Home,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GoToPage(childs_name) => {
                self.active_page = childs_name;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(Msg::GoToPage);

        html! {
            <HashRouter>
                <Nav {on_clicked} />
                <Switch<Route> render={switch} />
            </HashRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logging initialized");
    yew::Renderer::<App>::new().render();
}
