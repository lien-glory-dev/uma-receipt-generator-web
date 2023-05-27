use stylist::css;
use stylist::yew::{Global, use_style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::{switch, Route};
use crate::component::footer::Footer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <main>
                    <Switch<Route> render={switch} />
                </main>
                <footer>
                    <Footer />
                </footer>
            </BrowserRouter>
        </>
    }
}
