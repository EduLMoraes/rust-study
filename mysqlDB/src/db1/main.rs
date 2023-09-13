use mysql::Pool;
use chrono::Local;
use rand::Rng;

mod modules{
    pub mod search;
    pub mod insert_data;
    pub mod delete;
    pub mod edit;
}

use modules::*;
use search::*;
use insert_data::*;
use delete::*;

use crate::modules::edit::edit_name;
//use edit::*;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let pool = create_pool();

    let mut rng = rand::thread_rng();
    let id: i32 = rng.gen_range(1..=1000);

    let current_date = Local::now().date_naive();
    let date = current_date.format("%d/%m/%y").to_string();

    let data: (i32, String, String) = (id, date, "jhon".to_string());

    let insert = insert_data(&pool, &data)?;
    println!("\n");
    let result = select_all(&pool)?;
    println!("\n");
    let by_id = select_by_id(&pool, 03)?;
    println!("\n");
    let by_name = select_by_name(&pool, "Eduardo".to_string())?;
    println!("\n");
    let by_date = select_by_date(&pool, "25/08/2023".to_string())?;
    println!("\n");
    let delete_name = delete_from(&pool, "name".to_string(), "Eduardo".to_string())?;
    println!("\n");
    let delete_id = delete_from(&pool, "id".to_string(), "47".to_string())?;
    println!("\n");
    let edit_name = edit_name(&pool, id, "Paul".to_string())?;
    println!("\n");
    let result = select_all(&pool)?;
    println!("\n");

    println!("insert.....{}", insert);
    println!("select.....{}", result);
    println!("by id.....{}", by_id);
    println!("by name.....{}", by_name);
    println!("by date.....{}", by_date);
    //println!("delete by id.....{}", delete_id);
    //println!("delete by name.....{}", delete_name);
   println!("edit by name.....{}", edit_name);

    Ok(())
}

fn create_pool() -> Pool{
    let url: &str = "mysql://automato:0178@localhost:3306/study";
    Pool::new(url).expect("Failed to create pool")
}