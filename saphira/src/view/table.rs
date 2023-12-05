use dioxus::prelude::*;
use super::rentals::Rentals;

pub fn list_table<'a>(cx: Scope<'a>, rentals: &'a UseState<Rentals>) -> Element<'a>{
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