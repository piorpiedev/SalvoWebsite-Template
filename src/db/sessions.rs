use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use crate::{db::Conn, utils::random_string};

const SESSION_TOKEN_SIZE: usize = 32;

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token))
}

pub struct Session {
    pub expires_at: OffsetDateTime,
    pub token: String,
}

pub async fn create_session(conn: &Conn, user_id: i32) -> Result<Session, anyhow::Error> {
    let token = random_string(SESSION_TOKEN_SIZE);
    let token_hash = hash_token(&token);

    let expires_at = sqlx::query_scalar!(
        r#"
            INSERT INTO sessions (token_hash, user_id) 
            VALUES ($1,$2)
            RETURNING expires_at
        "#,
        &token_hash,
        user_id
    )
    .fetch_one(conn)
    .await?;

    Ok(Session { expires_at, token })
}

pub struct SessionStatus {
    pub expires_at: OffsetDateTime,
    pub user_id: i32,
}

pub async fn get_session(conn: &Conn, session_token: &str) -> Result<Option<SessionStatus>, anyhow::Error> {
    let token_hash: String = hash_token(session_token);

    let expires_at = sqlx::query_as!(
        SessionStatus,
        r#"
            SELECT expires_at, user_id
            FROM sessions
            WHERE token_hash = $1 AND expires_at > NOW()
            LIMIT 1
        "#,
        &token_hash
    )
    .fetch_optional(conn)
    .await?;

    Ok(expires_at)
}

pub async fn refresh_session(conn: &Conn, session_token: &str) -> Result<(), anyhow::Error> {
    let token_hash: String = hash_token(session_token);

    sqlx::query!(
        r#"
            UPDATE sessions
            SET expires_at = (NOW() + INTERVAL '30 days')
            WHERE token_hash = $1
        "#,
        token_hash
    )
    .execute(conn)
    .await?;

    Ok(())
}
