use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new().with_resizable(true).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(400.0, 800.0),
        )),
    );
}
