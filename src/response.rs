use crate::jwt::session::UserSession;
use actix_web::HttpResponse;
use std::time::SystemTime;

pub fn build_response(session: UserSession) -> HttpResponse {
    let jwt_cookie = format!(
        "jwt={}; Max-Age={}; Path=/; SameSite=strict; HttpOnly;",
        session.jwt_token, session.jwt_max_age_seconds
    );

    let refresh_cookie = format!(
        "refresh={}; Max-Age={}; Path=/; SameSite=strict; HttpOnly;",
        session.refresh, session.refresh_max_age_seconds
    );

    let unix_time_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    let jwt_expiry_value = (unix_time_secs + session.jwt_max_age_seconds) * 1000;

    let jwt_expiry_cookie = format!(
        "jwt_expiry={:?}; Max-Age={}; Path=/; SameSite=strict;",
        jwt_expiry_value, session.jwt_max_age_seconds
    );

    let refresh_expiry_value = (unix_time_secs + session.refresh_max_age_seconds) * 1000;

    let refresh_expiry_cookie = format!(
        "refresh_expiry={:?}; Max-Age={}; Path=/; SameSite=strict;",
        refresh_expiry_value, session.refresh_max_age_seconds
    );

    HttpResponse::Ok()
        .insert_header(("set-cookie", jwt_cookie))
        .append_header(("set-cookie", refresh_cookie))
        .append_header(("set-cookie", jwt_expiry_cookie))
        .append_header(("set-cookie", refresh_expiry_cookie))
        .finish()
}
