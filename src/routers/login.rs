use std::hint::black_box;

use askama::Template;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::Deserialize;
use validator::Validate;

use crate::{AppResult, db, hoops::auth, utils};

const FAILED_LOGIN_MSG: &str = "Account not exist or password is incorrect";
// If the user is not found, the suppllied password will be hashed against this (result is ignored)
const FAKE_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$fZHPU4UFZ+uzv5gZH7gAPQ$dGGTG7C+gNZDNGYcuuknxElKZM5WekmmWyCtNYVbYkk";

#[handler]
pub async fn handle_login_page(res: &mut Response) -> AppResult<()> {
    #[derive(Template)]
    #[template(path = "login.html")]
    struct LoginTemplate {}

    let hello_tmpl = LoginTemplate {};
    res.render(Text::Html(hello_tmpl.render().unwrap()));

    Ok(())
}

#[derive(Deserialize, ToSchema, Validate, Default, Debug)]
pub struct LoginData {
    #[validate(length(
        min = 3,
        max = 16,
        message = "Account not exist or password is incorrect"
    ))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 64,
        message = "Account not exist or password is incorrect"
    ))]
    pub password: String,
}

#[endpoint(tags("auth"))]
pub async fn handle_login_post(
    login_data: JsonBody<LoginData>,
    res: &mut Response,
) -> AppResult<()> {
    let login_data = login_data.into_inner();

    // Get user auth data
    let conn = db::pool();
    let Some(user) = db::users::get_user_auth(conn, &login_data.username).await? else {
        let _ = black_box(utils::verify_password(&login_data.password, FAKE_HASH)); // Make sure that we still try against something that will 100% not work
        return Err(StatusError::unauthorized().brief(FAILED_LOGIN_MSG).into());
    };

    // Verify password
    if utils::verify_password(&login_data.password, &user.password_hash).is_err() {
        return Err(StatusError::unauthorized().brief(FAILED_LOGIN_MSG).into());
    }

    // Add session token
    let session = db::sessions::create_session(conn, user.id).await?;
    let cookie = auth::gen_session_cookie(session.token, session.expires_at);
    res.add_cookie(cookie);

    Ok(())
}
