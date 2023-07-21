use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_film))
            .route("", web::post().to(new_film))
            .route("", web::put().to(put_film))
            .route("/{id}", web::delete().to(delete_film)),
    );
}

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn new_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn put_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}
