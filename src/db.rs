use sqlx::{Mssql, Pool};

pub type DbPool = Pool<Mssql>;

pub async fn create_pool() -> DbPool {
    let database_url = "mssql://sa:satan%40123@localhost:1433/TestDB?trustServerCertificate=true";

    Pool::<Mssql>::connect(database_url)
        .await
        .expect("Failed to connect to MS SQL Server")
}
