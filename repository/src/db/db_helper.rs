use std::error::Error;
use sqlx::postgres::PgPoolOptions;

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

    pub async fn init(&mut self) {
        let postgres_connection_string = format!(
            "postgres://{}:{}@{}/{}",
            self.user, self.password, self.db_host, self.db_name
        );

        let pool = PgPoolOptions::new()
            .max_connections(50)
            // .connect("postgres://myuser:mypassword@192.168.1.3/mydb")
            .connect(postgres_connection_string.as_str())
            .await;

        match pool {
            Ok(p) => {
                self.pool = Some(p);
            }
            Err(e) => {
                panic!("Database initialization error: {}", e);
            }
        }
    }

    pub fn get_pool(&self) -> Result<&sqlx::Pool<sqlx::Postgres>, Box<dyn Error>> {
        match self.pool.as_ref() {
            Some(pool) => Ok(pool),
            None => Err("Database not initialized".into())
        }
    }
}