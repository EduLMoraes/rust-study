use crate::equipment::Equipment;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use crate::rental::*;

pub fn new_rent(equip: String, lesson: bool, time: String) -> Rental{
    let mut equipment: Equipment;

    if lesson{
        match equip.to_lowercase().replace(" ", "").trim(){
            "jetski" => equipment = EquipmentWithLesson::new(1),
            "barcodepontao" => equipment = EquipmentWithLesson::new(2),
            "barcoaremo" => equipment = EquipmentWithLesson::new(3),
            "canoa" => equipment = EquipmentWithLesson::new(4),
            "caique" => equipment = EquipmentWithLesson::new(5),
            _ => equipment = EquipmentWithoutLesson::new(0)
        }
    }
    else {
        match equip.to_lowercase().replace(" ", "").trim(){
            "jetski" => equipment = EquipmentWithoutLesson::new(1),
            "barcodepontao" => equipment = EquipmentWithoutLesson::new(2),
            "barcoaremo" => equipment = EquipmentWithoutLesson::new(3),
            "canoa" => equipment = EquipmentWithoutLesson::new(4),
            "caique" => equipment = EquipmentWithoutLesson::new(5),
            "cadeiradepraia" => equipment = EquipmentWithoutLesson::new(6),
            "guardasol" => equipment = EquipmentWithoutLesson::new(7),
            "gazebo" => equipment = EquipmentWithoutLesson::new(8),
            _ => equipment = EquipmentWithoutLesson::new(0)
        }
    }

    Rental::new(time.parse::<i32>().unwrap(), equipment)
}