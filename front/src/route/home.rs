use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container page">
                <h1>{"因子結合くん"}</h1>
                <form action="/receipts" method="post" enctype="multipart/form-data">
                    <input type="checkbox" name="trim_margin" id="trim_margin" class="form-control" value="1"/>
                    <label for="trim_margin">{"余白を取り除く"}</label>
                    <input type="checkbox" name="trim_close_button" id="trim_close_button" class="form-control" value="1"/>
                    <label for="trim_close_button">{"「閉じる」ボタンを取り除く"}</label>
                    <input type="checkbox" name="trim_title" id="trim_title" class="form-control" value="1"/>
                    <label for="trim_title">{"「ウマ娘詳細」ヘッダーを取り除く"}</label>
                    <input type="file" multiple=true name="images[]"/>
                    <button type="submit">{"つなげる"}</button>
                </form>
            </div>
        }
    }
}
