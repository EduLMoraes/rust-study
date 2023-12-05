use dioxus::prelude::*;
use crate::control;
use crate::rentals::Rentals;
use crate::table::*;

pub fn app(cx: Scope) -> Element {

    let time = use_state(cx, || String::new());
    let equip = use_state(cx, || String::new());
    let lesson = use_state(cx, || false);
    let list = use_state(cx, || false);
    let rentals = use_state(cx, || Rentals::new());

    cx.render(rsx! {
       
        input {
            name: "time",
            oninput: move |evt| {
                time.set(evt.value.clone())
            }
        }

        input {
            name: "equip",
            oninput: move |evt| {
                equip.set(evt.value.clone())
            }
        }

        input { 
            r#type: "checkbox",
            onclick: move |_| 
            if !**lesson { lesson.set(true) }
            else { lesson.set(false) }
        }

        button { onclick: move |_| {
            rentals.set(
                control::add_rental(
                    control::get_id(equip.to_string()), 
                    time.parse::<i32>().unwrap(), 
                    **lesson
                )
            );

        }, "Confirmar Aluguel"},

        button { onclick: move |_| {
            list.set(true);
        }, "Listar Aluguéis"},
        
        if **list{
            list_table(&cx, rentals)    
        }
    })
}