use actix_web::{web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing::info!("Querying database");
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = api_lib::film_repository::PgFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(film_repository)
            .configure(api_lib::health_check::service)
            .configure(api_lib::films::service::<api_lib::film_repository::PgFilmRepository>);
    };

    Ok(config.into())
}
