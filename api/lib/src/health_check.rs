use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub const API_VERSION: &str = "v0.0.1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn health_check_returns_200() {
        let response = health().await;

        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);

        let data = response
            .headers()
            .get("version")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
