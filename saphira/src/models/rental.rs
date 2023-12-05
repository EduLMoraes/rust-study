use crate::equipment::Equipment;
use crate::equipment::{EquipmentWithLesson, EquipmentWithoutLesson};

pub struct Rental{
    contract: i64,
    time: i32,
    price: f32
}

impl Rental{
    pub fn new(time: i32, equip: Equipment) -> Self{
        Rental { contract: EquipmentWithoutLesson::get_type(), time: time, price: EquipmentWithLesson::get_value(time)}
    }

    pub fn total_price()
}