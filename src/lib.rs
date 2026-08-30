pub mod config;
pub mod core;
mod features;
pub use features::*;

use salvo::prelude::*;

#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct FrontendAssets;

pub fn create_service() -> Service {
    Service::new(router()).hoop(misc::cors_hoop()) // Applying to the service instead of the router so that OPTIONS preflight requests are handled automatically (https://salvo.rs/guide/features/cors.html)
}

fn router() -> Router {
    let mut router = Router::new().push(
        Router::with_path("/api")
            .hoop(Logger::new())
            .hoop(auth::auto_auth_middleware)
            .push(Router::with_path("login").post(auth::login_api))
            .push(
                Router::with_path("users")
                    .hoop(auth::require_auth_middleware)
                    .post(users::create_user_api)
                    .get(users::list_users_api)
                    .push(Router::with_path("@me").get(users::get_me_api))
                    .push(
                        Router::with_path("{user_id}")
                            .put(users::update_user_api)
                            .delete(users::delete_user_api),
                    ),
            ),
    );

    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router = router
        .unshift(doc.into_router("/api/api-doc/openapi.json"))
        .unshift(Scalar::new("/api/api-doc/openapi.json").into_router("api/scalar"));

    #[cfg(not(debug_assertions))]
    {
        use salvo::serve_static::static_embed;

        router = router.push(
            Router::with_path("{*path}")
                .get(static_embed::<FrontendAssets>().fallback("index.html")),
        );
    }
    router
}
