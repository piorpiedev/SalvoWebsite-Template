use cookie::Cookie;
use salvo::{Depot, FlowCtrl, Request, Response, handler, writing::Redirect};
use time::{Duration, OffsetDateTime};

use crate::{AppResult, config, db};

pub const COOKIE_NAME: &str = "session_token";
pub const SESSION_DURATION: Duration = Duration::days(30);

#[handler]
pub async fn handle_auto_auth(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    // Skip if not present
    let Some(cookie) = req.cookie(COOKIE_NAME) else {
        return Ok(());
    };

    // Get the associated user
    let session_token = cookie.value();
    let conn = db::pool();
    let Some(session) = db::sessions::get_session(conn, session_token).await? else {
        res.remove_cookie(COOKIE_NAME);
        return Ok(());
    };

    // Refresh session if needed
    let now = OffsetDateTime::now_utc();
    if session.expires_at - now < SESSION_DURATION / 2 {
        db::sessions::refresh_session(conn, session_token).await?;
        let mut cookie = cookie.clone();
        cookie.set_expires(now + SESSION_DURATION);
        res.add_cookie(cookie);
    }

    // Authenticate the user
    depot.insert("user.id", session.user_id);
    Ok(())
}

pub fn gen_session_cookie<'a>(session_token: String, expires_at: OffsetDateTime) -> Cookie<'a> {
    Cookie::build((COOKIE_NAME, session_token))
        .path("/")
        .same_site(cookie::SameSite::Strict)
        .expires(expires_at)
        .http_only(true)
        .secure(config::get().tls.enabled)
        .build()
}

#[handler]
pub async fn require_auth(depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if !depot.contains_key("user.id") {
        res.render(Redirect::other("/login"));
        ctrl.skip_rest();
    }
}
