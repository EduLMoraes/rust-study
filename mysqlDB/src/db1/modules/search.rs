use mysql::prelude::Queryable;
use mysql::{Pool, Statement};

pub fn select_all(pool: &Pool) -> Result<String, Box<dyn std::error::Error>>{
    let mut conn = pool.get_conn()?;
    let rows = conn.query("SELECT * FROM st1")?;

    for row in rows {
        let (id, create_time , name): (i32, String, String) = mysql::from_row(row);
        println!("ID: {}, Name {}, Create_time: {}", id, name, create_time);
    }

    Ok("ok".to_string())
}

pub fn select_by_id(pool: &Pool, id: i32) -> Result<String, Box<dyn std::error::Error>>{
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("SELECT * FROM st1 WHERE id = (?)")?;
    let rows = conn.exec::<(i32, String, String), Statement, _>(stmt, (id,))?;

    for row in rows {

        println!("ID: {}, Data Create: {}, Name: {}", row.0, row.1, row.2);
    }

    Ok("ok".to_string())
}

pub fn select_by_name(pool: &Pool, name: String) -> Result<String, Box<dyn std::error::Error>>{
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("SELECT * FROM st1 WHERE name = (?)")?;
    let rows = conn.exec::<(i32, String, String), Statement, _>(stmt, (name,))?;

    for row in rows {

        println!("ID: {}, Data Create: {}, Name: {}", row.0, row.1, row.2);
    }

    Ok("ok".to_string())
}

pub fn select_by_date(pool: &Pool, date: String) -> Result<String, Box<dyn std::error::Error>>{
    let mut conn = pool.get_conn()?;

    let stmt = conn.prep("SELECT * FROM st1 WHERE data_time = (?)")?;
    let rows = conn.exec::<(i32, String, String), Statement, _>(stmt, (date,))?;

    for row in rows {

        println!("ID: {}, Data Create: {}, Name: {}", row.0, row.1, row.2);
    }

    Ok("ok".to_string())
}