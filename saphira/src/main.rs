mod prelude;
use crate::prelude::{*, control::new_rent};

fn main() {
    // dioxus_desktop::launch_cfg(
    //     homepage::app,
    //     config::config(),
    // );
    let mut rentals: rentals::Rentals = rentals::Rentals::new();

    rentals.new_rental(1, 120, true);
    rentals.new_rental(1, 120, false);

    println!("{:?}", new_rent("Jetski".to_string(), true, "120".to_string()));
    println!("{:?}", new_rent("Jetski".to_string(), false, "120".to_string()));
    println!("{} ", rentals.list_all());
}
