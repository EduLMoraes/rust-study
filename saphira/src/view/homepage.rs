use dioxus::prelude::*;
use crate::control;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use crate::rentals::Rentals;


pub fn app(cx: Scope) -> Element {
    let mut teste = use_state(cx, || "alo".to_string());

    let time = use_state(cx, || "".to_string());
    let equip = use_state(cx, || "".to_string());
    let lesson = use_state(cx, || false);

    let value: i32;
    let mut rentals: Rentals = Rentals::new();

    cx.render(rsx! {
        form{
            onsubmit: move |event| {
                // rentals.new_rental(control::get_id(equip.to_string()), time.parse::<i32>().unwrap(), **lesson)
                teste.set(event.value.clone())
            },
            
            input {name: "time"}

            input {name: "equip"}

            //button { onclick: move |_| , "Confirmar Aluguel" }
            input { r#type: "checkbox", },

            input { r#type: "submit", },
        }
        
        label { "{teste} "}

        table{
            tr{
                td { format!("ID") }
                td { format!("Descrição") }
            }
            td { format!("{}", rentals.list_all()) }
        }
    })
}