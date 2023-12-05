use crate::rental::*;
use crate::equipment::*;

pub struct Rentals {
    rentals: Vec<Rental>,
}

impl Rentals{
    pub fn new() -> Self{
        Rentals{ rentals: Vec::new() }
    }

    pub fn new_rental(&mut self, id: i32, time: i32, has_lesson: bool) -> String{

        let mut equipment: Equipment;

        if has_lesson{
            equipment = EquipmentWithLesson::new(id);
        }
        else {
            equipment = EquipmentWithoutLesson::new(id);
        }
    
        let rental = Rental::new(time, equipment, has_lesson);
        
        let rent = &rental.to_string();
        self.rentals.push(rental);
        
        rent.to_string()
    }

    pub fn list_all(&self) -> String{
        let mut list = String::new();

        for rental in self.rentals.iter(){
            list.push_str(rental.to_string().trim());
        }

        list
    }
}