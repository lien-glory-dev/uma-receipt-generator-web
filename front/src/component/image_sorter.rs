use stylist::css;
use yew::prelude::*;

use crate::component::image_selector::Image;
use crate::component::sorting_image::SortingImage;

pub enum OrderChangedMessage {
    MoveLeft(usize),
    MoveRight(usize),
    Remove(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub images: Vec<Image>,
    pub loading_count: usize,
    pub on_change: Callback<OrderChangedMessage>,
    pub disabled: bool,
}

pub struct ImageSorter;

impl Component for ImageSorter {
    type Message = OrderChangedMessage;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().on_change.emit(msg);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let container_css = css! {"
            display: flex;
            width: 100%;
            height: 30rem;
            margin: 1.6rem 0;
            justify-content: center;
            
            .scroll-container {
                display: flex;
                max-width: 100%;
                height: 100%;
                overflow-x: scroll;
                overflow-y: hidden;
            }
        "};

        let loading_container_css = css! {"
            display: flex;
            flex-direction: column;
            flex-wrap: nowrap;
            width: 20rem;
            min-width: 20rem;
            margin: .5rem;
            padding: 0 .3rem;
            background-color: #303030;
            
            &:first-child {
                margin-left: 1rem;
            }
            &:last-child {
                margin-right: 1rem;
            }
        "};

        html! {
            <div class={container_css}>
                <div class="scroll-container" id="image_sorter_scroll">
                    { for ctx.props().images.iter().enumerate().map(|(index, image)| html! {
                        <SortingImage
                            index={index}
                            total_index={ctx.props().images.len()}
                            image={image.clone()}
                            on_click_left={ctx.link().callback(OrderChangedMessage::MoveLeft)}
                            on_click_right={ctx.link().callback(OrderChangedMessage::MoveRight)}
                            on_click_remove={ctx.link().callback(OrderChangedMessage::Remove)}
                            disabled={ctx.props().disabled}
                        />
                    }) }
                    { for (0..ctx.props().loading_count).map(|_| html! {
                        <div class={loading_container_css.clone()}>
                            <p>{"よみこみちう..."}</p>
                        </div>
                    })}
                </div>
            </div>
        }
    }
}
