use std::collections::HashMap;

use anyhow::anyhow;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::header::CONTENT_TYPE;
use reqwest::multipart::Part;
use stylist::css;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::component::button::*;
use crate::component::image_selector::*;
use crate::component::image_sorter::*;

pub enum Msg {
    AddImage(Image),
    ImageLoading(usize),
    ImageOrderChanged(OrderChangedMessage),
    ImageMerged(anyhow::Result<Image>),
    MergeImage,
    InputChanged(HtmlInputElement),
    ElementChanged(Event),
    BeginResultLoading,
    EndedResultLoading,
}

#[derive(Default)]
pub struct MergeForm {
    images: Vec<Image>,
    loading_count: usize,
    result_image: Option<Image>,
    is_loading_result: bool,
    check_options: HashMap<String, bool>,
}

impl Component for MergeForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            images: Vec::new(),
            loading_count: 0,
            result_image: None,
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let window = web_sys::window().expect("Failed to get window");

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
            Msg::MergeImage => {
                ctx.link().send_message(Msg::BeginResultLoading);
                
                let form = {
                    let f = self
                        .images
                        .iter()
                        .fold(reqwest::multipart::Form::new(), |f, image| {
                            let part = Part::bytes(image.bytes.clone())
                                .mime_str(image.mime_type.as_str())
                                .expect("Failed to set mime type");
                            f.part("images[]", part)
                        });

                    let f = self
                        .check_options
                        .iter()
                        .fold(f, |f, (option_name, is_checked)| {
                            let part = Part::text(is_checked.to_string())
                                .mime_str("text/plain")
                                .expect("Failed to set mime type");
                            f.part(option_name.clone(), part)
                        });

                    f
                };

                ctx.link().send_future(async {
                    let response = reqwest::Client::new()
                        .post(format!("{}/receipts", web_sys::window().unwrap().origin()))
                        .multipart(form)
                        .send()
                        .await
                        .map(|r| r.error_for_status());

                    let result = match response {
                        Ok(Ok(r)) => {
                            async {
                                let content_type = r
                                    .headers()
                                    .get(CONTENT_TYPE)
                                    .ok_or(anyhow!("Not found header 'Content-Type'"))?
                                    .to_str()
                                    .expect("Failed to convert header to str")
                                    .to_string();
                                let bytes = r.bytes().await?;

                                Ok(Image {
                                    name: "".to_string(),
                                    mime_type: content_type,
                                    size: bytes.len() as u64,
                                    bytes: bytes.to_vec(),
                                })
                            }
                            .await
                        }
                        Err(e) | Ok(Err(e)) => Err(e.into()),
                    };

                    Msg::ImageMerged(result)
                });
                false
            }
            Msg::ImageMerged(i) => {
                match i {
                    Ok(i) => {
                        self.result_image = Some(i);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("{:#?}", e).into());
                        window
                            .alert_with_message("画像の結合に失敗しました。")
                            .expect("Failed to alert");
                    }
                }
                
                ctx.link().send_message(Msg::EndedResultLoading);
                
                true
            }
            Msg::InputChanged(e) => {
                self.check_options.insert(e.name(), e.checked());
                false
            }
            Msg::ElementChanged(e) => {
                let input = e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .expect("It should input element");
                ctx.link().send_message(Msg::InputChanged(input));
                true
            }
            Msg::BeginResultLoading => {
                self.is_loading_result = true;
                true
            }
            Msg::EndedResultLoading => {
                self.is_loading_result = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let options_container_css = css! {"
            max-width: 25em;
            padding: 1.2rem;
            margin: 1.6rem auto;
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
        
        let button_container_css = css! {"
            margin: 1.6rem auto;
        "};

        let result_image_container_css = css! {"
            width: 100%;
            height: 40rem;
            margin: .6rem 0;
            flex: 1;
            
            img {
                width: 100%;
                height: 100%;
                object-fit: contain;
            }
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
                    disabled={self.is_loading_result}
                />
                <div class={options_container_css}>
                    <h1>{"オプション"}</h1>
                    <div class={options_group_css.clone()}>
                        <label for="trim_margin" class={options_item_css.clone()}>{"余白を取り除く"}</label>
                        <input type="checkbox" name="trim_margin" id="trim_margin" class={options_item_css.clone()} onchange={ctx.link().callback(Msg::ElementChanged)} />
                    </div>
                    <div class={options_group_css.clone()}>
                        <label for="trim_close_button" class={options_item_css.clone()}>{"「閉じる」ボタンを取り除く"}</label>
                        <input type="checkbox" name="trim_close_button" id="trim_close_button" class={options_item_css.clone()} onchange={ctx.link().callback(Msg::ElementChanged)} />
                    </div>
                    <div class={options_group_css.clone()}>
                        <label for="trim_title" class={options_item_css.clone()}>{"「ウマ娘詳細」ヘッダーを取り除く"}</label>
                        <input type="checkbox" name="trim_title" id="trim_title" class={options_item_css.clone()} onchange={ctx.link().callback(Msg::ElementChanged)} />
                    </div>
                </div>
                <div class={button_container_css}>
                    <Button
                        on_click={ctx.link().callback(|_| Msg::MergeImage)}
                        color={Color::Confirm}
                        disabled={self.is_loading_result}
                    >
                        {"つなげる"}
                    </Button>
                </div>
                if !self.is_loading_result {
                    if let Some(result_image) = &self.result_image {
                        <div class={result_image_container_css}>
                            <img src={format!("data:{};base64,{}", result_image.mime_type, STANDARD.encode(&result_image.bytes))} />
                        </div>
                    }
                } else {
                    <div class={result_image_container_css}>
                        <p>{"がっちゃんこちう..."}</p>
                    </div>
                }
                <div class="container footer-buttons">
                    <Button>{"使い方"}</Button>
                    <Button>{"うまくいかない時"}</Button>
                </div>
            </div>
        }
    }
}

impl MergeForm {

}