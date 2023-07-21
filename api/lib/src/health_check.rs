use actix_web::{get, web};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(database: web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");

    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(database.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Database Error: {:?}", e),
    }
}
