use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;

pub mod home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <>{"NotFound"}</> },
    }
}
