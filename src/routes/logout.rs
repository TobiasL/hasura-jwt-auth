use actix_web::HttpResponse;

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("set-cookie", "jwt=; Max-Age=-1; Path=/;"))
        .append_header(("set-cookie", "refresh=; Max-Age=-1; Path=/;"))
        .append_header(("set-cookie", "jwt_expiry=; Max-Age=-1; Path=/;"))
        .append_header(("set-cookie", "refresh_expiry=; Max-Age=-1; Path=/;"))
        .finish()
}
