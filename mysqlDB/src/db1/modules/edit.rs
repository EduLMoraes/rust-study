use mysql::{self, Statement};
use mysql::prelude::Queryable;
use mysql::Pool;

pub fn edit_name(pool: &Pool, id: i32, name: String) -> Result<String, Box<dyn std::error::Error>>{
    
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("UPDATE st1 SET name = ? WHERE id = ?")?;
    conn.exec::<(i32, String), Statement, _>(stmt, (name, id))?;

    Ok("ok".to_string())

}