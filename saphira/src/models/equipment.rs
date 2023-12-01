use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Equipment{
    id: i32,
    description: String,
    equipment_and_values: EquipmentAndValues
}pub trait EquipmentWithoutLesson{
    fn new(id: i32) -> Self;
    fn get_value(&self, time: i32) -> f32;
    fn get_equipment_and_values() -> EquipmentAndValues;
    fn get_description() -> String;
    fn get_type() -> i32;
    fn to_string() -> String;
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
        let rate = match self.equipment_and_values{
            EquipmentAndValues::EQUIPMENT(_, rate, _, _) => {
                rate
            }
        };

        let time: f32 = time as f32 / 60.0;

        println!("{:?}", rate * time);
        4.0
    }
    fn get_equipment_and_values() -> EquipmentAndValues{
        EquipmentAndValues::EIGHT()
    }
    fn get_description() -> String{
        "ola".to_string()
    }
    fn get_type() -> i32{
        4
    }
    fn to_string() -> String{
        "String".to_string()
    }
}

impl EquipmentWithLesson for Equipment{
    fn new(id: i32) -> Self{
        match id{
            1 => Equipment {id: id, description: "Jet Ski".to_string(), equipment_and_values: EquipmentAndValues::ONE()},
            2 => Equipment {id: id, description: "Barco de pontão".to_string(), equipment_and_values: EquipmentAndValues::TWO()},
            3 => Equipment {id: id, description: "Barco a remo".to_string(), equipment_and_values: EquipmentAndValues::THREE()},
            4 => Equipment {id: id, description: "Canoa".to_string(), equipment_and_values: EquipmentAndValues::FOUR()},
            _ => Equipment {id: id, description: "Caique".to_string(), equipment_and_values: EquipmentAndValues::FIVE()}
        }   
    }

    fn get_value(&self, time: i32) -> f32{
        let (rate_base, rate) = match self.equipment_and_values{
            EquipmentAndValues::EQUIPMENT(_, rate_base, rate, _) => {
                (rate_base, rate)
            }
        };
        let time: f32 = time as f32 / 60.0;

        (rate_base + (rate * time)) + 20.0
    }
}

#[derive(Serialize, Debug)]
enum EquipmentAndValues{
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