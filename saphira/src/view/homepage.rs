use dioxus::prelude::*;

#[path = "../controller/control.rs"]
mod control;

pub fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let mut rent = control::new_rent();

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Confirmar Aluguel" }
        button { onclick: move |_| count -= 1, "Listar Aluguéis" }
    })
}