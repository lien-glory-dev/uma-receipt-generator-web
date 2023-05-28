use std::ops::Deref;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use stylist::css;
use yew::prelude::*;

use crate::component::button::{Button, Color};
use crate::component::image_selector::Image;
use crate::component::image_sorter::OrderChangedMessage;

pub enum Msg {
    OrderChanged(OrderChangedMessage),
    Encoding,
    Encoded(AttrValue),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image: Image,
    pub index: usize,
    pub total_index: usize,
    #[prop_or_default]
    pub class: Classes,
    pub on_click_left: Callback<usize>,
    pub on_click_right: Callback<usize>,
    pub on_click_remove: Callback<usize>,
    pub disabled: bool,
}

pub struct SortingImage {
    encoded_image: AttrValue,
    is_encoding: bool,
}

impl Component for SortingImage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            encoded_image: AttrValue::Static(""),
            is_encoding: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OrderChanged(msg) => match msg {
                OrderChangedMessage::MoveLeft(i) => {
                    ctx.props().on_click_left.emit(i);
                    true
                }
                OrderChangedMessage::MoveRight(i) => {
                    ctx.props().on_click_right.emit(i);
                    true
                }
                OrderChangedMessage::Remove(i) => {
                    ctx.props().on_click_remove.emit(i);
                    true
                }
            },
            Msg::Encoding => {
                let image = ctx.props().image.clone();

                self.is_encoding = true;

                ctx.link().send_future(async move {
                    let e = Self::encode_image(image).await;
                    Msg::Encoded(e)
                });
                true
            }
            Msg::Encoded(encoded) => {
                self.encoded_image = encoded;
                self.is_encoding = false;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if !ctx
            .props()
            .image
            .bytes
            .borrow()
            .eq(_old_props.image.bytes.borrow().deref())
        {
            ctx.link().send_message(Msg::Encoding);
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let container_css = css! {"
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

        let header_css = css! {"
            width: 100%;
            
            h1 {
                margin: 0;
                font-size: 1.2rem;
                line-height: 2em;
            }
            p {
                margin: 0;
                font-size: .8rem;
                line-height: 1em;
            }
        "};

        let image_container_css = css! {"
            width: 100%;
            height: 60%;
            margin: .6rem 0;
            flex: 1;
            
            img {
                width: 100%;
                height: 100%;
                object-fit: contain;
            }
        "};

        let footer_css = css! {"
            margin: .2rem 0;
        "};

        let index = ctx.props().index;
        let size_mega_byte = ctx.props().image.size as f64 / 1000000.0;

        html! {
            <div class={classes!(container_css, ctx.props().class.clone())}>
                <div class={header_css}>
                    <h1>{ctx.props().index + 1} {"枚目"}</h1>
                    <p>{format!("{:.2} MB", size_mega_byte)}</p>
                </div>
                <div class={image_container_css}>
                    // TODO: コンポーネント化する。Base64Image みたいな。エンコードもそっちでやる
                    if !self.encoded_image.is_empty() {
                        <img src={&self.encoded_image} />
                    } else {
                        <p>{"プレビュー生成ちう..."}</p>
                    }
                </div>
                <div class={footer_css}>
                    <Button
                        on_click={ctx.link().callback(move |_| Msg::OrderChanged(OrderChangedMessage::MoveLeft(index)))}
                        disabled={ctx.props().index == 0 || ctx.props().disabled}
                    >
                        {"←"}
                    </Button>
                    <Button
                        on_click={ctx.link().callback(move |_| Msg::OrderChanged(OrderChangedMessage::Remove(index)))}
                        color={Color::Error}
                        disabled={ctx.props().disabled}
                    >
                        {"削除"}
                    </Button>
                    <Button
                        on_click={ctx.link().callback(move |_| Msg::OrderChanged(OrderChangedMessage::MoveRight(index)))}
                        disabled={ctx.props().index >= ctx.props().total_index - 1 || ctx.props().disabled}
                    >
                        {"→"}
                    </Button>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if !self.is_encoding && self.encoded_image.is_empty() {
            ctx.link().send_message(Msg::Encoding);
        }
    }
}

impl SortingImage {
    async fn encode_image(image: Image) -> AttrValue {
        format!(
            "data:{};base64,{}",
            image.mime_type,
            STANDARD.encode(image.bytes.borrow().deref())
        )
        .into()
    }
}
