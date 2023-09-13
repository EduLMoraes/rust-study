use mysql::{self, Statement};
use mysql::prelude::Queryable;
use mysql::Pool;

pub fn delete_from(pool: &Pool, column: String, value: String) -> Result<String, Box<dyn std::error::Error>>{
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep(&format!("DELETE FROM st1 WHERE {} = (?)", column))?;

    if column == "id".to_string(){
        let value: i32 = value.trim().parse().unwrap();
        conn.exec::<(i32, String, String), Statement, _>(stmt, (value,))?;
    }
    else{
        conn.exec::<(i32, String, String), Statement, _>(stmt, (value,))?;
    }

    Ok("ok".to_string())
}