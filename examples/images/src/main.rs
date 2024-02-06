mod app;
mod pages;
mod router;

fn main() {
    next_rs::Renderer::<app::App>::new().render();
}
