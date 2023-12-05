use dioxus::prelude::*;
use crate::control;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use crate::rentals::Rentals;


pub fn app(cx: Scope) -> Element {
    let mut teste = use_state(cx, ||String::new());

    let time = use_state(cx, || "".to_string());
    let equip = use_state(cx, || "".to_string());
    let lesson = use_state(cx, || false);

    let value: i32;
    let mut rentals: Rentals = Rentals::new();

    cx.render(rsx! {
        form{
            onsubmit: move |_| {
                rentals.new_rental(control::get_id(equip.to_string()), time.parse::<i32>().unwrap(), **lesson);
                teste.set(rentals.list_all())
            },
            
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

            //button { onclick: move |_| , "Confirmar Aluguel" }
            input { 
                r#type: "checkbox",
                onclick: move |_| 
                if !**lesson { lesson.set(true) }
                else { lesson.set(false) },
                if **lesson {"✅"}
                else {"❌"}
            }

            input { r#type: "submit", },
        }
        
        label { "{teste} "}

        table{
            tr{
                td { format!("ID") }
                td { format!("Descrição") }
            }
            td {  }
        }
    })
}