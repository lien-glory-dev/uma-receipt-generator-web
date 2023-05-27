use std::collections::HashMap;

use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_change: Callback<Image>,
    #[prop_or_default]
    pub on_loading: Callback<usize>,
}

pub enum Msg {
    FileReady(Image),
    ImagesSelected(Vec<File>),
    FilesSelectionFailed,
}

pub struct ImageSelector {
    files_value: AttrValue,
    readers: HashMap<String, FileReader>,
}

impl Component for ImageSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            files_value: Default::default(),
            readers: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let window = web_sys::window().expect("Failed to get window");

        match msg {
            Msg::FileReady(image) => {
                self.readers.remove(image.name.as_str());
                ctx.props().on_change.emit(image);
                true
            }
            Msg::ImagesSelected(images) => {
                for file in images.into_iter() {
                    let file_name = file.name();
                    let file_byte_size = file.size();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();
                        
                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::FileReady(Image {
                                name: file_name,
                                mime_type: file_type,
                                size: file_byte_size,
                                bytes: res.expect("Failed to read file"),
                            }))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                
                self.files_value = "".into();
                
                ctx.props().on_loading.emit(self.readers.len());
                true
            }
            Msg::FilesSelectionFailed => {
                window
                    .alert_with_message("ファイルの読み込みに失敗しました。")
                    .expect("Failed to alert");

                false
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
        let input: HtmlInputElement = {
            let r = e.target_dyn_into::<HtmlInputElement>();

            if r.is_none() {
                return Msg::FilesSelectionFailed;
            }

            r.unwrap()
        };

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
    pub bytes: Vec<u8>,
}
