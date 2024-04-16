use crate::handlers::auth;
use crate::models;

use actix_web::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        auth::refresh,
        auth::logout,
    ),
    components(schemas(
            models::User,
            models::SignUpCredentials,
            models::SignInCredentials,
    )),
    tags(
        (name = "auth", description = "Authorization API")
    ),
)]
struct ApiDocs;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        utoipa_rapidoc::RapiDoc::with_openapi(
            "/api-docs/openapi.json",
            ApiDocs::openapi(),
        )
        .path("/docs"),
    )
    .service(auth::register)
    .service(auth::login)
    .service(auth::refresh)
    .service(auth::logout);
}