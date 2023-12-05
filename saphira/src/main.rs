mod prelude;
use prelude::*;
fn main() {
    dioxus_desktop::launch_cfg(
        homepage::app,
        config::config(),
    );

    let mut rentals = rentals::Rentals::new();
    rentals.new_rental(1, 120, true);
    rentals.new_rental(1, 120, true);
    rentals.new_rental(1, 120, true);
    rentals.new_rental(1, 120, true);
    rentals.new_rental(1, 120, true);

    let string = format!("{}", rentals.list_all());
    println!("{string}");
}
