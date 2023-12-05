use crate::equipment::Equipment;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};

pub fn new_rent(equip: String, lesson: bool, time: String) -> Equipment{
    let mut rent: Equipment;

    if lesson{
        match equip.to_lowercase().replace(" ", "").trim(){
            "jetski" => rent = EquipmentWithLesson::new(1),
            "barcodepontao" => rent = EquipmentWithLesson::new(2),
            "barcoaremo" => rent = EquipmentWithLesson::new(3),
            "canoa" => rent = EquipmentWithLesson::new(4),
            "caique" => rent = EquipmentWithLesson::new(5),
            _ => rent = EquipmentWithoutLesson::new(0)
        }
    }
    else {
        match equip.to_lowercase().replace(" ", "").trim(){
            "jetski" => rent = EquipmentWithoutLesson::new(1),
            "barcodepontao" => rent = EquipmentWithoutLesson::new(2),
            "barcoaremo" => rent = EquipmentWithoutLesson::new(3),
            "canoa" => rent = EquipmentWithoutLesson::new(4),
            "caique" => rent = EquipmentWithoutLesson::new(5),
            "cadeiradepraia" => rent = EquipmentWithoutLesson::new(6),
            "guardasol" => rent = EquipmentWithoutLesson::new(7),
            "gazebo" => rent = EquipmentWithoutLesson::new(8),
            _ => rent = EquipmentWithoutLesson::new(0)
        }
    }

    rent
}