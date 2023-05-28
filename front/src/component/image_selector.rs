use std::cell::RefCell;
use std::rc::Rc;

use gloo::file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_change: Callback<Image>,
    #[prop_or_default]
    pub on_loading: Callback<usize>,
    #[prop_or_default]
    pub on_failed: Callback<()>,
}

pub enum Msg {
    FileReady(Image),
    ImagesSelected(Vec<File>),
    FileLoadError,
}

pub struct ImageSelector {
    files_value: AttrValue,
}

impl Component for ImageSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files_value: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let window = web_sys::window().expect("Failed to get window");

        match msg {
            Msg::FileReady(image) => {
                ctx.props().on_change.emit(image);
                true
            }
            Msg::ImagesSelected(images) => {
                ctx.props().on_loading.emit(images.len());

                for file in images.into_iter() {
                    let file_name = file.name();
                    let file_byte_size = file.size();
                    let file_type = file.raw_mime_type();

                    ctx.link().send_future(async move {
                        let bytes = gloo::file::futures::read_as_bytes(&file).await;

                        match bytes {
                            Ok(bytes) => Msg::FileReady(Image {
                                name: file_name,
                                mime_type: file_type,
                                size: file_byte_size,
                                bytes: Rc::new(RefCell::new(bytes)),
                            }),
                            Err(_) => Msg::FileLoadError,
                        }
                    });
                }

                self.files_value = "".into();

                true
            }
            Msg::FileLoadError => {
                window
                    .alert_with_message("ファイルの読み込みに失敗しました。")
                    .expect("Failed to alert");

                ctx.props().on_failed.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container image-selector">
                <input
                    type="file"
                    multiple=true
                    name="images[]"
                    value={self.files_value.clone()}
                    onchange={ctx.link().callback(Self::on_change)}
                />
            </div>
        }
    }
}

impl ImageSelector {
    fn on_change(e: Event) -> Msg {
        let input: HtmlInputElement = e.target_dyn_into().expect("It should input element");

        let mut result_files = Vec::new();

        if let Some(files) = input.files() {
            let (selected_images, others): (Vec<_>, Vec<_>) = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from)
                .partition(|f| f.raw_mime_type() == "image/png");

            if !others.is_empty() {
                web_sys::window()
                    .expect("Failed to get window")
                    .alert_with_message("対応していないファイルがありました。")
                    .expect("Failed to alert");
            }

            result_files.extend(selected_images);
        }

        Msg::ImagesSelected(result_files)
    }
}

#[derive(PartialEq, Clone)]
pub struct Image {
    pub name: String,
    pub mime_type: String,
    pub size: u64,
    pub bytes: Rc<RefCell<Vec<u8>>>,
}
