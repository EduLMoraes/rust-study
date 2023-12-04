use equipment::{Equipment, EquipmentWithLesson, EquipmentWithoutLesson};
use dioxus::prelude::*;
use dioxus::html::*;

#[path="./models/equipment.rs"]
mod equipment;

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    })
}

fn main() {
    dioxus_desktop::launch(app);
    
}
