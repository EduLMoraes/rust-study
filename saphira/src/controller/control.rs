#[path = "../models/equipment.rs"]
mod equipment;
use equipment::Equipment;

use self::equipment::EquipmentWithLesson;

pub fn new_rent() -> Equipment{

    EquipmentWithLesson::new(1)
}