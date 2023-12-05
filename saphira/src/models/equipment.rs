use serde::Serialize;
use rand::{self, Rng};

#[derive(Serialize, Debug)]
pub struct Equipment{
    id: i32,
    description: String,
    equipment_and_values: EquipmentAndValues
}pub trait EquipmentWithoutLesson{
    fn new(id: i32) -> Self;
    fn get_value(&self, time: i32) -> f32;
    fn get_equipment_and_values(&self) -> EquipmentAndValues;
    fn get_description(&self) -> String;
    fn get_type(&self) -> i32;
    fn to_string(&self) -> String;
}pub trait EquipmentWithLesson: EquipmentWithoutLesson{

    fn new(id: i32) -> Self;
    fn get_value(&self, time: i32) -> f32;
}

impl EquipmentWithoutLesson for Equipment{
    fn new(id: i32) -> Self{
        match id{
            1 => Equipment {id: id, description: "Jet Ski".to_string(), equipment_and_values: EquipmentAndValues::ONE()},
            2 => Equipment {id: id, description: "Barco de pontão".to_string(), equipment_and_values: EquipmentAndValues::TWO()},
            3 => Equipment {id: id, description: "Barco a remo".to_string(), equipment_and_values: EquipmentAndValues::THREE()},
            4 => Equipment {id: id, description: "Canoa".to_string(), equipment_and_values: EquipmentAndValues::FOUR()},
            5 => Equipment {id: id, description: "Caique".to_string(), equipment_and_values: EquipmentAndValues::FIVE()},
            6 => Equipment {id: id, description: "Cadeira de praia".to_string(), equipment_and_values: EquipmentAndValues::SIX()},
            7 => Equipment {id: id, description: "Guarda-sol".to_string(), equipment_and_values: EquipmentAndValues::SEVEN()},
            _ => Equipment {id: id, description: "Gazebo".to_string(), equipment_and_values: EquipmentAndValues::EIGHT()}
        }   
    }

    fn get_value(&self, time: i32) -> f32{
        let (rate_base, rate) = match self.equipment_and_values{
            EquipmentAndValues::EQUIPMENT(_, rate_base, rate, _) => {
                (rate_base, rate)
            }
        };

        let time: f32 = time as f32 / 60.0;

        rate_base + (rate * time)
    }
    fn get_equipment_and_values(&self) -> EquipmentAndValues{
        let (desc, rate_base, rate, has_lesson) = match &self.equipment_and_values{
            EquipmentAndValues::EQUIPMENT(desc, rate_base, rate, has_lesson) => {
                (desc, rate_base, rate, has_lesson)
            }
        };

        EquipmentAndValues::EQUIPMENT(desc.to_string(), *rate_base, *rate, *has_lesson)
    }
    fn get_description(&self) -> String{
        match self.get_equipment_and_values(){
            EquipmentAndValues::EQUIPMENT(desc, _, _, _) => {
                desc
            }
        }
    }
    fn get_type(&self) -> i32{
        rand::thread_rng().gen_range(0..9999)
    }
    fn to_string(&self) -> String{
        format!("{} {} {:?}", self.id, self.description, self.get_equipment_and_values())
    }
}

impl EquipmentWithLesson for Equipment{
    fn new(id: i32) -> Self{
        match id{
            1 => Equipment {id: id, description: "Jet Ski".to_string(), equipment_and_values: EquipmentAndValues::ONE()},
            2 => Equipment {id: id, description: "Barco de pontão".to_string(), equipment_and_values: EquipmentAndValues::TWO()},
            3 => Equipment {id: id, description: "Barco a remo".to_string(), equipment_and_values: EquipmentAndValues::THREE()},
            4 => Equipment {id: id, description: "Canoa".to_string(), equipment_and_values: EquipmentAndValues::FOUR()},
            _ => Equipment {id: 5, description: "Caique".to_string(), equipment_and_values: EquipmentAndValues::FIVE()}
        }   
    }

    fn get_value(&self, time: i32) -> f32{
        let (rate_base, rate, has_lesson) = match self.equipment_and_values{
            EquipmentAndValues::EQUIPMENT(_, rate_base, rate, has_lesson) => {
                (rate_base, rate, has_lesson)
            }
        };
        
        let mut value = 0.0;

        if has_lesson {
            value = 20.0;
        }

        let time: f32 = time as f32 / 60.0;

        (rate_base + (rate * time)) + value
    }
}

#[derive(Serialize, Debug)]
pub enum EquipmentAndValues{
    EQUIPMENT(String, f32, f32, bool),
}

#[allow(dead_code)]
impl EquipmentAndValues{
    #[allow(non_snake_case)]
    fn ONE() -> Self{
        EquipmentAndValues::EQUIPMENT("Jet Ski".to_string(), 50.0, 30., true)
    }
    #[allow(non_snake_case)]
    fn TWO() -> Self{
        EquipmentAndValues::EQUIPMENT("Barco de pontão".to_string(), 40.0, 30.0, true)
    }
    #[allow(non_snake_case)]
    fn THREE() -> Self{
        EquipmentAndValues::EQUIPMENT("Barco a remo".to_string(), 15.0, 20.0, true)
    }
    #[allow(non_snake_case)]
    fn FOUR() -> Self{
        EquipmentAndValues::EQUIPMENT("Canoa".to_string(), 12.0, 20.0, true)
    }
    #[allow(non_snake_case)]
    fn FIVE() -> Self{
        EquipmentAndValues::EQUIPMENT("Caique".to_string(), 10.0, 20.0, true)
    }
    #[allow(non_snake_case)]
    fn SIX() -> Self{
        EquipmentAndValues::EQUIPMENT("Cadeira de praia".to_string(), 2.0, 5.0, false)
    }
    #[allow(non_snake_case)]
    fn SEVEN() -> Self{
        EquipmentAndValues::EQUIPMENT("Guarda Sol".to_string(), 1.0, 5.0, false)
    }
    #[allow(non_snake_case)]
    fn EIGHT() -> Self{
        EquipmentAndValues::EQUIPMENT("Gazebo".to_string(), 3.0, 7.0, false)
    }
}