use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;

use home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home title="ウマウマがっちゃんこ" /> },
        Route::NotFound => html! { <>{"NotFound"}</> },
    }
}
