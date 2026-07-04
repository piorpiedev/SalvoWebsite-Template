use crate::db::Conn;

pub struct UserAuth {
    pub id: i32,
    pub password_hash: String,
}

pub async fn get_user_auth(conn: &Conn, username: &str) -> Result<Option<UserAuth>, anyhow::Error> {
    let user = sqlx::query_as!(
        UserAuth,
        r#"
            SELECT id, password_hash
            FROM users
            WHERE username = $1
            LIMIT 1
        "#,
        username
    )
    .fetch_optional(conn)
    .await?;

    Ok(user)
}

pub async fn create_user(
    conn: &Conn,
    username: &str,
    password_hash: &str,
) -> Result<i32, anyhow::Error> {
    let user_id = sqlx::query_scalar!(
        r#"
            INSERT INTO users (username, password_hash)
            VALUES ($1, $2)
            RETURNING id
        "#,
        username,
        password_hash
    )
    .fetch_one(conn)
    .await?;

    Ok(user_id)
}

pub async fn update_user(
    conn: &Conn,
    user_id: i32,
    username: &str,
    password_hash: &str,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
            UPDATE users
            SET username = $1, password_hash = $2
            WHERE id = $3
        "#,
        username,
        password_hash,
        user_id,
    )
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn delete_user(conn: &Conn, user_id: i32) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
            DELETE FROM users
            WHERE id = $1
        "#,
        user_id,
    )
    .execute(conn)
    .await?;

    Ok(())
}
