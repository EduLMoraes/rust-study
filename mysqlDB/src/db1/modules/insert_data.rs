use mysql::{self, Statement};
use mysql::prelude::Queryable;
use mysql::Pool;

pub fn insert_data(pool: &Pool, data: &(i32, String, String)) -> Result<String, Box<dyn std::error::Error>>{
    let mut conn = pool.get_conn()?;

    let id = &data.0;
    let data_create = &data.1;
    let name = &data.2;    

    let stmt = conn.prep("INSERT INTO st1 (id, data_time, name) VALUES (?, ?, ?)")?;
    conn.exec::<(i32, String, String), Statement, _>(stmt, (id, data_create, name))?;

    Ok("ok".to_string())
}
