use sea_orm::{Database, DatabaseConnection};

use crate::migration;

pub async fn init_test_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:")
        .await
        .expect("database connection failed");

    migration::up(&db).await.expect("migrate failed");

    db
}
