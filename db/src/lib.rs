use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn migration() -> Result<(), MigrateError> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@db/mydb").await?;

    sqlx::migrate!("../misc/migrations")
        .run(&pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
