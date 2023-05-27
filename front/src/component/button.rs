use stylist::{css, StyleSource};
use stylist::yew::use_style;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Color {
    White,
    Confirm,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_else(|| Color::White)]
    pub color: Color,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let button_css = css! {
        font-size: ${"1rem"};
        padding: ${".7em 2em"};
        margin: ${".2em .4em"};
        border-style: none;
        border-radius: ${".4em"};
        line-height: ${"1em"};
        
        transition-property: background;
        transition-duration: 0.1s;

        cursor: pointer;
        
        &:disabled {
            cursor: not-allowed;
            background-color: ${props.color.get_disabled_color()};
        }
    };
    
    html! {
        <button class={classes!(button_css.clone(), props.color.get_color_style(), props.class.clone())} onclick={&props.onclick} disabled={props.disabled}>
            { for props.children.iter() }
        </button>
    }
}

impl Color {
    pub fn get_primary_color(&self) -> String {
        match self {
            Color::White => "#fbfbfc",
            Color::Confirm => "#4db0ff",
            Color::Error => "#db5c5c",
        }.to_string()
    }
    
    pub fn get_secondary_color(&self) -> String {
        match self {
            Color::White => "#c8c8c9",
            Color::Confirm => "#3d8ccc",
            Color::Error => "#a74646",
        }.to_string()
    }
    
    pub fn get_disabled_color(&self) -> String {
        match self {
            Color::White => "#aeaeaf",
            Color::Confirm => "#357bb2",
            Color::Error => "#8e3b3b",
        }.to_string()
    }
    
    pub fn get_foreground_color(&self) -> String {
        match self {
            Color::White => "#000",
            Color::Confirm => "#fff",
            Color::Error => "#fff",
        }.to_string()
    }
    
    fn get_color_style(&self) -> StyleSource {
        css! {
            background-color: ${self.get_primary_color()};
            color: ${self.get_foreground_color()};
            
            &:hover {
                background-color: ${self.get_secondary_color()};
            }
        }
    }
}