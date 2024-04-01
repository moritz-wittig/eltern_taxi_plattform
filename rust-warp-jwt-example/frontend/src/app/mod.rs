pub mod login;
pub mod settings;
use login::Login;
use settings::Settings;
use yew::prelude::*;
use yew_router::prelude::*;


/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Login,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Settings => html!(<Settings/>),
        AppRoute::NotFound => html! { "Page not found" },
    }
}

/// The root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}