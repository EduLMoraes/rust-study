use dioxus::prelude::*;
use super::rentals::get_rentals_instance;

pub fn list_table(cx: Scope) -> Element{
    let rentals = use_state(cx, || get_rentals_instance());

    cx.render(rsx!{
        table {
            tr{
                td{ "Contrato" },
                td{ "Tempo de aluguél" },
                td{ "Preço" }
            }
            for rental in rentals.rentals.clone(){
                tr{
                    td{ format!("{}", rental.contract) },
                    td{ format!("{}", rental.time) },
                    td{ format!("{}", rental.price) }
                }
            }
        }
    })
}