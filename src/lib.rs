pub mod config;
pub mod core;
mod features;
pub use features::*;

use rust_embed::RustEmbed;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use salvo::serve_static::{EmbeddedFileExt, static_embed};

use crate::misc::error_page;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn create_service() -> Service {
    Service::new(router())
        .catcher(Catcher::default().hoop(error_page))
        .hoop(misc::cors_hoop()) // Applying to the service instead of the router so that OPTIONS preflight requests are handled automatically (https://salvo.rs/guide/features/cors.html)
}

fn router() -> Router {
    let favicon = Assets::get("favicon.ico")
        .expect("favicon not found")
        .into_handler();
    let router = Router::new()
        .hoop(Logger::new())
        .hoop(auth::auto_auth_middleware)
        .get(homepage::handle_homepage_page)
        .push(Router::with_path("login").get(auth::handle_login_page))
        .push(
            Router::with_path("users")
                .hoop(auth::require_auth_middleware)
                .get(users::list_users_page),
        )
        .push(
            Router::with_path("api")
                .push(Router::with_path("login").post(auth::handle_login_post))
                .push(
                    Router::with_path("users")
                        .hoop(auth::require_auth_middleware)
                        .get(users::list_users_api)
                        .post(users::create_user_api)
                        .push(
                            Router::with_path("{user_id}")
                                .put(users::update_user_api)
                                .delete(users::delete_user_api),
                        ),
                ),
        )
        .push(Router::with_path("favicon.ico").get(favicon))
        .push(Router::with_path("assets/{**rest}").get(static_embed::<Assets>()));
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
