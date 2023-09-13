use tokio;

mod db;

#[tokio::main]
async fn main() {
    let database = db::DataBase::new("mysql://automato:0178@localhost:3306/study");

    let result = database.search_all().await.unwrap();
    for row in result{
        let (id, data, name): (i32, String, String) = mysql::from_row(row);
        println!("{}, {}, {}", id, data, name);
    }
    
    println!("{:?}", database.search_for("Paul".to_string()).await.unwrap());
}