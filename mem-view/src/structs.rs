
use mem_dbg::*;

#[derive(Clone, Debug, PartialEq)]
#[derive(MemSize, MemDbg)]

pub struct User {
    pub id: i32,
    pub debtor: String,
    pub title: String,
    pub description: String,
    pub value: f32,
    pub date_in: String,
    pub date_out: String,
    pub paid_installments: u32,
    pub installments: u32,
    pub status: bool,
}

impl User {
    pub fn new() -> User {

        User {
            id: format!("{}0{}", 14, 0).trim().parse::<i32>().unwrap(),
            debtor: String::new(),
            title: String::new(),
            description: String::new(),
            value: 0.0,
            date_in: "2023-02-18".to_string(),
            date_out: "2023-02-18".to_string(),
            paid_installments: 0,
            installments: 1,
            status: false,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
#[derive(MemSize, MemDbg)]
pub struct UserList {
    pub list: Vec<User>,
}

impl UserList {
    #[allow(dead_code)]
    pub fn new() -> UserList {
        UserList { list: Vec::new() }
    }

    pub fn put(&mut self, value: User) {
        self.list.insert(0, value)
    }

}