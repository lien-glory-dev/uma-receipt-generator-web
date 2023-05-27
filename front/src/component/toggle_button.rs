use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub onchange: Callback<Event>,
    pub class: Classes,
    pub name: AttrValue,
    pub id: AttrValue,
    pub value: AttrValue,
}

#[function_component(ToggleButton)]
pub fn toggle_button(props: &Props) -> Html {
    let toggle_button_css = use_style! {"
        display: none;
        
        &:after {
            width: 2rem;
            border-radius: 9999px;
            background-color: #fff;
        }
    "};

    html! {
        <input type="checkbox" class={classes!(toggle_button_css.clone(), props.class.clone())} name={&props.name} id={&props.id} value={&props.value} onchange={&props.onchange} />
    }
}
