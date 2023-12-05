use dioxus::prelude::*;
use crate::control;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use crate::equipment::Equipment;


pub fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let time = use_state(cx, || "".to_string());
    let equip = use_state(cx, || "".to_string());
    let lesson = use_state(cx, || false);

    let value: i32;
    let mut rentals: Vec<Equipment> = vec![control::new_rent(equip.to_string(), **lesson, time.to_string())];

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

        button { onclick: move |_| rentals.push(control::new_rent(equip.to_string(), **lesson, time.to_string())), "Confirmar Aluguel" }
        button { onclick: move |_| count -= 1, "Listar Aluguéis" }

        table{
            tr{
                td { format!("ID") }
                td { format!("Descrição") }
            }
            td { format!("{:?}", rentals) }

            for rent in rentals.iter_mut() {
                tr{
                    td { format!("{:?}", rent) }
                    // td { format!("{}", EquipmentWithoutLesson::get_description(&rentals[0])) }
    
                }
            }
        }
    })
}