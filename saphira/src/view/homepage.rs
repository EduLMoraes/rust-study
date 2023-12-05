use dioxus::prelude::*;
use crate::control;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use crate::rentals::Rentals;


pub fn app(cx: Scope) -> Element {
    let mut rentals = Rentals::new();

    let time = use_state(cx, || String::new());
    let equip = use_state(cx, || String::new());
    let lesson = use_state(cx, || false);
    let texto = use_state(cx, || String::new());

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
            rentals.new_rental(control::get_id(equip.to_string()), time.trim().parse::<i32>().unwrap(), **lesson);
            texto.set(rentals.list_all());
        }, "new rental"},
        
        label {
            format!("{} {}", texto, equip.to_string())
        }
         
    })
}