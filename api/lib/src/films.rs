use crate::film_repository::FilmRepository;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all::<R>))
            .route("/{id}", web::get().to(get_film::<R>))
            .route("", web::post().to(new_film::<R>))
            .route("", web::put().to(put_film::<R>))
            .route("/{id}", web::delete().to(delete_film::<R>)),
    );
}

async fn get_all<R: FilmRepository>(repository: web::Data<R>) -> HttpResponse {
    match repository.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal Server Error: {:?}", e)),
    }
}

async fn get_film<R: FilmRepository>(
    film_id: web::Path<Uuid>,
    repository: web::Data<R>,
) -> HttpResponse {
    match repository.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

async fn new_film<R: FilmRepository>(
    create_film: web::Json<CreateFilm>,
    repository: web::Data<R>,
) -> HttpResponse {
    match repository.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {:?}", e))
        }
    }
}

async fn put_film<R: FilmRepository>(
    film: web::Json<Film>,
    repository: web::Data<R>,
) -> HttpResponse {
    match repository.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal Server Error: {:?}", e)),
    }
}

async fn delete_film<R: FilmRepository>(
    film_id: web::Path<Uuid>,
    repository: web::Data<R>,
) -> HttpResponse {
    match repository.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {:?}", e))
        }
    }
}
