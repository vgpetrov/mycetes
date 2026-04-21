use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, warn};
use anyhow::{Result, Context};

pub struct DbHelper {
    pool: Option<sqlx::Pool<sqlx::Postgres>>,
    user: String,
    password: String,
    db_host: String,
    db_name: String,
}

impl DbHelper {
    pub fn new(user: String, password: String, db_host: String, db_name: String) -> Self {
        DbHelper {
            pool: None,
            user,
            password,
            db_host,
            db_name,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        let postgres_connection_string = format!(
            "postgres://{}:{}@{}/{}",
            self.user, self.password, self.db_host, self.db_name
        );

        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(&postgres_connection_string)
            .await
            .context("Failed to initialize database connection")?;

        self.pool = Some(pool);
        Ok(())
    }

    pub fn get_pool(&self) -> Result<&sqlx::Pool<sqlx::Postgres>, Box<dyn Error>> {
        match self.pool.as_ref() {
            Some(pool) => Ok(pool),
            None => Err("Database not initialized".into())
        }
    }
}