use app::App;

mod app;
mod component;
mod route;

fn main() {
    yew::Renderer::<App>::new().render();
}
