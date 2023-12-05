use crate::equipment::*;
use crate::rental::Rental;

pub fn new_rent(desc: String, lesson: bool, time: String) -> Rental{
    let mut equip: Equipment;

    if lesson{
        match desc.to_lowercase().replace(" ", "").trim(){
            "jetski" => equip = EquipmentWithLesson::new(1),
            "barcodepontao" => equip = EquipmentWithLesson::new(2),
            "barcoaremo" => equip = EquipmentWithLesson::new(3),
            "canoa" => equip = EquipmentWithLesson::new(4),
            "caique" => equip = EquipmentWithLesson::new(5),
            _ => equip = EquipmentWithoutLesson::new(0)
        }
    }
    else {
        match desc.to_lowercase().replace("", " ").trim(){
            "jetski" => equip = EquipmentWithoutLesson::new(1),
            "barcodepontao" => equip = EquipmentWithoutLesson::new(2),
            "barcoaremo" => equip = EquipmentWithoutLesson::new(3),
            "canoa" => equip = EquipmentWithoutLesson::new(4),
            "caique" => equip = EquipmentWithoutLesson::new(5),
            "cadeiradepraia" => equip = EquipmentWithoutLesson::new(6),
            "guardasol" => equip = EquipmentWithoutLesson::new(7),
            "gazebo" => equip = EquipmentWithoutLesson::new(8),
            _ => equip = EquipmentWithoutLesson::new(0)
        }
    }

    Rental::new(time.parse::<i32>().unwrap(), equip)
}