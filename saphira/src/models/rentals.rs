use crate::rental::*;
use crate::equipment::*;
use serde::Serialize;
use std::io::Write;
use std::fs::File;
use lazy_static::lazy_static;
use std::sync::{Mutex};

#[derive(Serialize, Clone, Debug)]
pub struct Rentals {
    pub rentals: Vec<Rental>,
}

impl Rentals{
    pub fn new() -> Self{
        Rentals{ rentals: Vec::new() }
    }

    pub fn new_rental(&mut self, id: i32, time: i32, has_lesson: bool) -> String{

        let equipment: Equipment;

        if has_lesson{
            equipment = EquipmentWithLesson::new(id);
        }
        else {
            equipment = EquipmentWithoutLesson::new(id);
        }
    
        
        self.rentals.push(Rental::new(time, equipment, has_lesson));
        
        let rent = self.rentals[0];

        rent.to_string()

    }
    pub fn save_to_file(&self, name: String){
        let data = self.list_all();

        let mut file = match File::create(name.clone()) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Erro ao criar o arquivo: {}", e);
                return;
            }
        };

        match file.write_all(data.as_bytes()) {
            Ok(_) => println!("Dados escritos com sucesso!"),
            Err(e) => eprintln!("Erro ao escrever no arquivo: {}", e),
        }
    }
    pub fn list_all(&self) -> String{
        let mut list = String::new();

        for rental in self.rentals.iter(){
            list.push_str(rental.to_string().trim());
            list.push_str("\n");
        }

        list
    }
}

// Cria uma variável global de tipo Mutex que contém uma instância de Rentals
lazy_static! {
    static ref GLOBAL_RENTALS: Mutex<Rentals> = Mutex::new(Rentals::new());
}

// Função para acessar a instância única de Rentals
pub fn get_rentals_instance() -> std::sync::MutexGuard<'static, Rentals> {
    GLOBAL_RENTALS.lock().unwrap()
}