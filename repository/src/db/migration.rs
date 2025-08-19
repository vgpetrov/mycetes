use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;

pub async fn migrate_files(
    user: String,
    password: String,
    db_host: String,
    db_name: String,
) -> Result<(), MigrateError> {
    let postgres_connection_string = format!("postgres://{}:{}@{}/{}", user, password, db_host, db_name);

    let pool = PgPoolOptions::new()
        .max_connections(1)
        // .connect("postgres://myuser:mypassword@192.168.1.3/mydb")
        .connect(postgres_connection_string.as_str())
        .await?;

    sqlx::migrate!("../misc/migrations").run(&pool).await?;

    Ok(())
}
