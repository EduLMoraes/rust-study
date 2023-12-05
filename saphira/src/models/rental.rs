use crate::equipment::Equipment;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson, EquipmentAndValues};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Rental{
    contract: i64,
    time: i32,
    price: f32
}

impl Rental{
    pub fn new(time: i32, equip: Equipment, has_lesson: bool) -> Self{
        Rental { 
            contract: equip.get_type() as i64, 
            time: time, 
            price: if has_lesson{
                EquipmentWithLesson::get_value(&equip, time)
            }else{
                EquipmentWithoutLesson::get_value(&equip, time)
            }
        }
    }

    pub fn total_price(&self) -> f32 {
        self.price
    }

    pub fn to_string(&self) -> String{
        format!("{} {} {}", self.contract, self.time, self.price)
    }
}