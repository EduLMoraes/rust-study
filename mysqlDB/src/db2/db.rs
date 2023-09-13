use mysql::{*, prelude::Queryable};

#[derive(Debug)]
pub struct DataBase<'a>{
    url: &'a str,
}

impl <'a> DataBase<'a>{
    pub fn new(url: &str) -> DataBase{
        DataBase{ url: url }
    }

    async fn get_pool(&self) -> Result<Pool, DataBaseError>{
        let pool = Pool::new(self.url)?;
        Ok(pool)
    }

    pub async fn search_all(&self) -> Result<Vec<Row>, DataBaseError>{
        let pool = self.get_pool().await?;
        let mut conn = pool.get_conn()?;
    
        let rows: Vec<Row> = conn.query("SELECT * FROM st1")?;
        
        Ok(rows)
    }

    pub async fn search_for(&self, name: String) -> Result<Vec<Row>, DataBaseError>{
        let pool = self.get_pool().await?;
        let mut conn = pool.get_conn()?;

        let stmt = conn.prep("SELECT * FROM st1 WHERE name = ?")?;
        let row = conn.exec(stmt, (name,))?;

        Ok(row)
    }

    
}



#[derive(Debug)]
pub enum DataBaseError{
    ConnectionFailed { field1: String },
    QueryError(mysql::Error)
}

impl From<String> for DataBaseError {
    fn from(field1: String) -> Self {
        Self::ConnectionFailed { field1 }
    }
}

impl From<mysql::Error> for DataBaseError {
    fn from(err: mysql::Error) -> Self{
        DataBaseError::QueryError(err)
    }
}