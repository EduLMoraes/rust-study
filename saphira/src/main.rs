mod prelude;
use crate::prelude::*;

fn main() {
    // dioxus_desktop::launch_cfg(
    //     homepage::app,
    //     config::config(),
    // );
    let mut rentals: Vec<rental::Rental> = Vec::new();
    rentals.push(control::new_rent("JETSKI".to_string(), false, "120".to_string()));
    rentals.push(control::new_rent("JETSKI".to_string(), false, "120".to_string()));
    rentals.push(control::new_rent("JETSKI".to_string(), false, "120".to_string()));
    rentals.push(control::new_rent("JETSKI".to_string(), false, "120".to_string()));
    rentals.push(control::new_rent("JETSKI".to_string(), false, "120".to_string()));

    for rent in rentals.iter_mut() {
        println!("{:?}", rent);
    }
}
