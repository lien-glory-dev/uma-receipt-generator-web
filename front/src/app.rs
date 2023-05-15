use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        </>
    }
}
