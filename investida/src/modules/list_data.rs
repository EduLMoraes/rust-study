extern crate csv;

use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::io::Result;
use csv::Reader;

pub fn list_data() {
    
    python_rmv();

    let file_path = Path::new("./src/data/new_fills.csv");
    let file = File::open(file_path).expect("Falha ao abrir o arquivo");
    let mut reader = Reader::from_reader(file);


    let mut fills: Vec<Vec<String>> = Vec::new();

    fills.push(vec!["Olá".to_string()]);
    println!("{:?}", fills);

    for result in reader.records(){
        match result{
            Ok(record) => {
                let _coluna1 = &record[0];

            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

fn python_rmv()-> Result<()>{
    Command::new("python").arg("./src/rmv_acent.py").output()?;
            
    Ok(())
}