use dioxus::prelude::*;
use crate::control;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use crate::rentals::Rentals;


pub fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let time = use_state(cx, || "".to_string());
    let equip = use_state(cx, || "".to_string());
    let lesson = use_state(cx, || false);

    let value: i32;
    let mut rentals: Rentals = Rentals::new();

    cx.render(rsx! {
        label { "Tempo de Alguel" }
        input {value: "{time}", oninput: move |evt| time.set(evt.value.clone())}
        br {}

        label { "Equipamento" }
        input {value: "{equip}", oninput: move |evt| equip.set(evt.value.clone())}
        br {}

        label { "Incluir Aula" }
        button { onclick: move |_| 
            if !**lesson { lesson.set(true) } else { lesson.set(false) }, 
            if **lesson {"✅"} else {"❌"}
        }
        br {}

        button { onclick: move |_| rentals.new_rental(control::get_id(equip.to_string()), time.parse::<i32>().unwrap(), **lesson), "Confirmar Aluguel" }
        button { onclick: move |_| count -= 1, "Listar Aluguéis" }

        table{
            tr{
                td { format!("ID") }
                td { format!("Descrição") }
            }
            td { format!("{:?}", rentals.list_all()) }
        }
    })
}