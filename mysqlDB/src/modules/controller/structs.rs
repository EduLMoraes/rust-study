use rocket::{Data, FromForm};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Book{
    pub id: i32,
    pub publisher: String,
    pub author: String,
    pub title: String,
    pub category: String,
    pub edition: String,
    pub date: String,
    pub description: String,
    pub qnt: i32
}

#[path="../db/db.rs"]
mod db;
use db::*;

#[derive(Serialize, Deserialize)]
pub struct User{
    pub email: String,
    pub id: i32,
    pub permission: String,
    pub book_allocated: String,
    pub name: String,
    pub surname: String,
}
impl User {
    pub fn new(data_user: Vec<String>) -> User{
        User { 
            email: data_user[0].clone(), 
            id: data_user[1].trim().parse::<i32>().unwrap(), 
            permission: data_user[2].clone(),
            book_allocated: data_user[3].clone(),
            name: data_user[4].clone(),
            surname: data_user[5].clone(),
        }
    }
}

struct Admin;
struct Librarian;

trait UserPermissions{
    fn search();
    fn connect_db();
    fn order_by(); 
} trait LibrarianPermissions: UserPermissions{
    fn add_book();
    fn edit_qnt();
} trait AdminPermissions: LibrarianPermissions{
    fn delete();
    fn edit();
}



#[derive(FromForm)]
pub struct LoginForm{
    pub email: String,
    pub password: String,
}

