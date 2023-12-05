mod prelude;
use prelude::*;
fn main() {
    // dioxus_desktop::launch_cfg(
    //     homepage::app,
    //     config::config(),
    // );
    let mut rentals: rentals::Rentals = rentals::Rentals::new();

    rentals.new_rental(1, 120, true);
    rentals.new_rental(1, 120, false);

    println!("{} ", rentals.list_all());
}
