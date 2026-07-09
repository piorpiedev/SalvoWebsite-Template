use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::{
    core::{
        database,
        error::{AppResult, JsonResult},
        utils,
    },
    users::db,
};

// #[handler]
// pub async fn list_users_page(req: &mut Request, res: &mut Response) {
//     let is_fragment = req.headers().get("X-Fragment-Header");
//     match is_fragment {
//         Some(_) => {
//             render_template!(res, "user_list_frag.html");
//         }
//         None => {
//             render_template!(res, "user_list_page.html");
//         }
//     }
// }

#[derive(Deserialize, Validate, ToSchema)]
pub struct UserUpdateData {
    #[validate(length(
        min = 3,
        max = 16,
        message = "Username must be between 3 and 16 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 64,
        message = "Password must be between 8 and 64 characters"
    ))]
    pub password: String,
}
#[derive(ToSchema, Serialize, Debug, TS)]
#[ts(export)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
}

#[endpoint(tags("users"))]
pub async fn create_user_api(idata: JsonBody<UserUpdateData>) -> JsonResult<UserInfo> {
    let UserUpdateData { username, password } = idata.into_inner();
    let password_hash = utils::hash_password(&password)?;

    let conn = database::pool();
    let user_id = db::create_user(conn, &username, &password_hash).await?;

    Ok(Json(UserInfo {
        id: user_id,
        username,
    }))
}

#[endpoint(tags("users"), parameters(("user_id", description = "user id")))]
pub async fn update_user_api(
    user_id: PathParam<i32>,
    idata: JsonBody<UserUpdateData>,
) -> JsonResult<UserInfo> {
    let user_id = user_id.into_inner();
    let UserUpdateData { username, password } = idata.into_inner();
    let password_hash = utils::hash_password(&password)?;

    let conn = database::pool();
    db::update_user(conn, user_id, &username, &password_hash).await?;

    Ok(Json(UserInfo {
        id: user_id,
        username,
    }))
}

#[endpoint(tags("users"))]
pub async fn delete_user_api(user_id: PathParam<i32>) -> AppResult<()> {
    let user_id = user_id.into_inner();

    let conn = database::pool();
    db::delete_user(conn, user_id).await?;

    Ok(())
}

#[derive(Debug, Deserialize, Validate, Extractible, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
#[serde(default)]
pub struct UserListQuery {
    pub username: Option<String>,
    pub current_page: i64,
    pub page_size: i64,
}
impl Default for UserListQuery {
    fn default() -> Self {
        Self {
            username: None,
            current_page: 1,
            page_size: 10,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub data: Vec<UserInfo>,
    pub total: i64,
    pub current_page: i64,
    pub page_size: i64,
}

#[endpoint(tags("users"))]
pub async fn list_users_api(
    query: &mut Request,
    depot: &mut Depot,
) -> JsonResult<UserListResponse> {
    let conn = database::pool();
    let query: UserListQuery = query.extract(depot).await?;
    let username_filter = query.username.clone().unwrap_or_default();
    let like_pattern = format!("%{}%", username_filter);
    let offset = (query.current_page - 1) * query.page_size;

    let total = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) as "count!: i64" FROM users
            WHERE username LIKE $1
        "#,
        like_pattern
    )
    .fetch_one(conn)
    .await?;

    let users = sqlx::query_as!(
        UserInfo,
        r#"
            SELECT id, username FROM users
            WHERE username LIKE $1
            LIMIT $2 OFFSET $3
        "#,
        like_pattern,
        query.page_size,
        offset
    )
    .fetch_all(conn)
    .await?;

    Ok(Json(UserListResponse {
        data: users,
        total,
        current_page: query.current_page,
        page_size: query.page_size,
    }))
}

#[endpoint(tags("users"))]
pub async fn get_me_api(depot: &mut Depot) -> JsonResult<UserInfo> {
    let user_id = depot.get::<i32>("user.id").copied().map_err(|e| match e {
        None => StatusError::unauthorized(),
        Some(_) => StatusError::internal_server_error(),
    })?;

    let pool = database::pool();
    let Some(user) = db::get_user_info(pool, user_id).await? else {
        return Err(StatusError::internal_server_error().into());
    };

    Ok(Json(UserInfo {
        id: user_id,
        username: user.username,
    }))
}
