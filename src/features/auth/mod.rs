use cookie::Cookie;
use time::{Duration, OffsetDateTime};

use crate::config;

pub mod db;
mod handlers;
mod middleware;

pub use handlers::*;
pub use middleware::*;

pub const COOKIE_NAME: &str = "session_token";
pub const SESSION_DURATION: Duration = Duration::days(30);

pub fn gen_session_cookie<'a>(session_token: String, expires_at: OffsetDateTime) -> Cookie<'a> {
    Cookie::build((COOKIE_NAME, session_token))
        .path("/")
        .same_site(cookie::SameSite::Lax)
        .expires(expires_at)
        .http_only(true)
        .secure(config::get().tls.enabled)
        .build()
}
