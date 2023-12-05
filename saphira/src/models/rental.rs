use crate::equipment::Equipment;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Rental{
    pub contract: i64,
    pub time: i32,
    pub price: f32
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

    #[allow(dead_code)]
    pub fn total_price(&self) -> f32 {
        self.price
    }

    pub fn to_string(&self) -> String{
        format!("{} {} {}", self.contract, self.time, self.price)
    }
}