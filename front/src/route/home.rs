use stylist::yew::use_style;
use yew::prelude::*;

use crate::component::merge_form::MergeForm;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    let page_container_css = use_style! {"
        margin-left: auto;
        margin-right: auto;
        text-align: center;
    "};

    html! {
        <div class={page_container_css}>
            <div class="container title">
                <h1>{&props.title}</h1>
            </div>
            <MergeForm />
        </div>
    }
}
