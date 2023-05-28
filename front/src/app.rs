use yew::prelude::*;
use yew_router::prelude::*;

use crate::component::footer::Footer;
use crate::route::{switch, Route};

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
