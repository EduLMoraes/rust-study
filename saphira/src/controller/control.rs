use crate::equipment::*;
use crate::rental::Rental;

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