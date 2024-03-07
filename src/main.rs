use colors::Colors;
use floem::{
    self,
    view::View,
    views::{h_stack, text, v_stack, Decorators},
};

pub mod colors;

pub const MAIN_WINDOW_TITLE: &str = "Floem UI Demo";
pub const HELLO_WORLD: &str = "Hello, World!";

pub fn hello_world() -> impl View {
    text(HELLO_WORLD).style(|style| style.color(Colors::WHITE))
}

pub fn app_view() -> impl View {
    v_stack((h_stack((hello_world(),)),))
        .style(|style| {
            style
                .flex_col()
                .width_full()
                .height_full()
                .background(Colors::BLACK)
        })
        .window_title(|| String::from(MAIN_WINDOW_TITLE))
}

fn main() {
    floem::launch(app_view);
}
