pub mod login;
use login::Login;
use yew::prelude::*;
use yew_router::prelude::*;


/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}