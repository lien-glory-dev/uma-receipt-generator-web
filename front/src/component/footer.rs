use stylist::yew::use_style;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let container_css = use_style! {"
        p {
        }
    "};
    
    html! {
        <div class={container_css}>
            <p>{"© 2023 リヤングローリー All rights reserved."}</p>
        </div>
    }
}