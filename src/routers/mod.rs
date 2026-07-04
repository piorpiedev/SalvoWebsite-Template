use rust_embed::RustEmbed;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use salvo::serve_static::{EmbeddedFileExt, static_embed};

mod demo;
mod login;
mod user;

use crate::hoops::{self, handle_404};

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn create_service() -> Service {
    Service::new(root()).catcher(Catcher::default().hoop(handle_404))
}

fn root() -> Router {
    let favicon = Assets::get("favicon.ico")
        .expect("favicon not found")
        .into_handler();
    let router = Router::new()
        .hoop(Logger::new())
        .hoop(hoops::cors::cors_hoop())
        .hoop(hoops::auth::handle_auto_auth)
        .hoop(hoops::handle_404)
        .get(demo::hello)
        .push(Router::with_path("login").get(login::handle_login_page))
        .push(
            Router::with_path("users")
                .hoop(hoops::auth::require_auth)
                .get(user::list_page),
        )
        .push(
            Router::with_path("api")
                .push(Router::with_path("login").post(login::handle_login_post))
                .push(
                    Router::with_path("users")
                        .hoop(hoops::auth::require_auth)
                        .get(user::list_users)
                        .post(user::create_user)
                        .push(
                            Router::with_path("{user_id}")
                                .put(user::update_user)
                                .delete(user::delete_user),
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
