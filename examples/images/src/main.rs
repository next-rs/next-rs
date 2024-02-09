mod app;
mod pages;

fn main() {
    next_rs::Renderer::<app::App>::new().render();
}
