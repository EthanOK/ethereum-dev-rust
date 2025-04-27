use eyre::Result;
use sea_orm::{Database, DatabaseConnection};

pub async fn get_mysql_connection_env() -> Result<DatabaseConnection> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("MYSQL_DATABASE_URL").expect("MYSQL_DATABASE_URL must be set");
    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub async fn get_mysql_connection(database_url: &str) -> Result<DatabaseConnection> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}
