use gloo::utils::document;
use stylist::yew::use_style;
use yew::prelude::*;

use crate::component::merge_form::MergeForm;

#[function_component(Home)]
pub fn home() -> Html {
    let page_container_css = use_style! {"
        margin-left: auto;
        margin-right: auto;
        text-align: center;
    "};
    
    let title = document().title();

    html! {
        <div class={page_container_css}>
            <div class="container title">
                <h1>{title}</h1>
            </div>
            <MergeForm />
        </div>
    }
}
