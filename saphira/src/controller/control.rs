use crate::rentals::*;

pub fn get_id(desc: String) -> i32{
    match desc.to_lowercase().replace(" ", "").trim(){
        "jetski" => 1,
        "barcodepontao" => 2,
        "barcoaremo" => 3,
        "canoa" => 4,
        "caique" => 5,
        "cadeiradepraia" => 6,
        "guardasol" => 7,
        "gazebo" => 8,
        _ => 0
    }
}

pub fn add_rental( id: i32, time: i32, lesson: bool) -> Rentals{
    let mut rentals = get_rentals_instance();

    rentals.new_rental(id, time, lesson);
    rentals.save_to_file("a.txt".to_string());

    rentals.clone()
}