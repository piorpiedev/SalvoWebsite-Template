use salvo::{Depot, FlowCtrl, Request, Response, handler, writing::Redirect};
use time::OffsetDateTime;

use crate::{
    auth::{COOKIE_NAME, SESSION_DURATION, db},
    core::{database, error::AppResult},
};

#[handler]
pub async fn auto_auth_middleware(
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
    let conn = database::pool();
    let Some(session) = db::get_session(conn, session_token).await? else {
        res.remove_cookie(COOKIE_NAME);
        return Ok(());
    };

    // Refresh session if needed
    let now = OffsetDateTime::now_utc();
    if session.expires_at - now < SESSION_DURATION / 2 {
        db::refresh_session(conn, session_token).await?;
        let mut cookie = cookie.clone();
        cookie.set_expires(now + SESSION_DURATION);
        res.add_cookie(cookie);
    }

    // Authenticate the user
    depot.insert("user.id", session.user_id);
    Ok(())
}

#[handler]
pub async fn require_auth_middleware(depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if !depot.contains_key("user.id") {
        res.render(Redirect::other("/login"));
        ctrl.skip_rest();
    }
}
