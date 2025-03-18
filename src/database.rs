use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing::info;

pub(crate) const DB_URL: &'static str = "sqlite://sqlite.db";

pub(crate) async fn create_pool() -> SqlitePool {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database {}", DB_URL);

        match Sqlite::create_database(DB_URL).await {
            Ok(_) => info!("Creating DB was successful"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let cwd = std::env::current_dir().unwrap();
    let crate_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or(String::from(cwd.to_str().unwrap()));
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => info!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    db
}
