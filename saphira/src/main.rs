use equipment::{Equipment, EquipmentWithLesson};

#[path="./models/equipment.rs"]
mod equipment;

fn main() {
    let a: Equipment = EquipmentWithLesson::new(3);
    println!("{}", a.get_value(120));
}
