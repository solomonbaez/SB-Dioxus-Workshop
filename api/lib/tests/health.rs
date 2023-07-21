use actix_web::{http::StatusCode, App};
use api_lib::health_check::{service, API_VERSION};

#[actix_rt::test]
async fn health_check_returns_200() {
    let app = App::new().configure(service);
    let mut app = actix_web::test::init_service(app).await;

    let request = actix_web::test::TestRequest::get()
        .uri("/health")
        .to_request();

    let response = actix_web::test::call_service(&mut app, request).await;

    assert!(response.status().is_success());
    assert_eq!(response.status(), StatusCode::OK);

    let data = response
        .headers()
        .get("version")
        .and_then(|h| h.to_str().ok());

    assert_eq!(data, Some(API_VERSION));
}
