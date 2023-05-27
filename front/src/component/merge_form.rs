use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use stylist::css;
use yew::prelude::*;

use crate::component::button::{Button, Color};
use crate::component::image_selector::{Image, ImageSelector};
use crate::component::image_sorter::*;

pub enum Msg {
    AddImage(Image),
    ImageLoading(usize),
    ImageOrderChanged(OrderChangedMessage),
}

pub struct MergeForm {
    images: Vec<Image>,
    loading_count: usize,
    result_image: Option<Image>,
}

impl Component for MergeForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            images: Vec::new(),
            loading_count: 0,
            result_image: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddImage(i) => {
                self.images.push(i);
                self.loading_count -= 1;
                true
            }
            Msg::ImageLoading(count) => {
                self.loading_count += count;
                true
            }
            Msg::ImageOrderChanged(msg) => match msg {
                OrderChangedMessage::MoveLeft(i) => {
                    self.images.swap(i, i - 1);
                    true
                }
                OrderChangedMessage::MoveRight(i) => {
                    self.images.swap(i, i + 1);
                    true
                }
                OrderChangedMessage::Remove(i) => {
                    self.images.remove(i);
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let options_container_css = css! {"
            max-width: 25em;
            padding: 1.2rem;
            margin-top: 1.6rem;
            margin-bottom: 1.6rem;
            margin-left: auto;
            margin-right: auto;
            border-color: #666;
            border-width: 1px;
            border-style: solid;
            background-color: #333;
            text-align: left;
            
            h1 {
                font-size: 1.2rem;
            }
        "};

        let options_group_css = css! {"
            display: flex;
            justify-content: between;
            align-items: center;
            
            label {
                display: flex;
                flex-grow: 1;
            }
            input {
                display: flex;
            }
        "};

        let options_item_css = css! {"
            font-size: 1rem;
            line-height: 1.8em;
        "};

        html! {
            <div class="container stylist-Nl4fCjeC">
                <ImageSelector
                    on_change={ctx.link().callback(Msg::AddImage)}
                    on_loading={ctx.link().callback(Msg::ImageLoading)}
                />
                <ImageSorter
                    images={self.images.clone()}
                    loading_count={self.loading_count}
                    on_change={ctx.link().callback(Msg::ImageOrderChanged)}
                />
                <div class="container button-area">
                    <Button color={Color::Confirm}>{"つなげる"}</Button>
                </div>
                <div class="container result-image-area">
                    if let Some(result_image) = &self.result_image {
                        <img src={format!("data:{};base64,{}", result_image.mime_type, STANDARD.encode(&result_image.bytes))} />
                    }
                </div>
                <div class={options_container_css}>
                    <h1>{"オプション"}</h1>
                    <div class={options_group_css.clone()}>
                        <label for="trim_margin" class={options_item_css.clone()}>{"余白を取り除く"}</label>
                        <input type="checkbox" name="trim_margin" id="trim_margin" class={options_item_css.clone()} value="1" />
                    </div>
                    <div class={options_group_css.clone()}>
                        <label for="trim_close_button" class={options_item_css.clone()}>{"「閉じる」ボタンを取り除く"}</label>
                        <input type="checkbox" name="trim_close_button" id="trim_close_button" class={options_item_css.clone()} value="1" />
                    </div>
                    <div class={options_group_css.clone()}>
                        <label for="trim_title" class={options_item_css.clone()}>{"「ウマ娘詳細」ヘッダーを取り除く"}</label>
                        <input type="checkbox" name="trim_title" id="trim_title" class={options_item_css.clone()} value="1" />
                    </div>
                </div>
                <div class="container footer-buttons">
                    <Button>{"使い方"}</Button>
                    <Button>{"うまくいかない時"}</Button>
                </div>
            </div>
        }
    }
}
