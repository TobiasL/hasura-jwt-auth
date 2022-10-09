use actix_web::HttpResponse;

pub async fn live() -> HttpResponse {
    HttpResponse::Ok().finish()
}
