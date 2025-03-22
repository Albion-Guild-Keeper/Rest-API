use actix_web::HttpRequest;

use crate::database::connect as database;
use crate::utils::surreal_int::SurrealInt;

pub async fn join_user(
    id: SurrealInt,
    username: String,
    joined_at: String,
    discord_id: SurrealInt,
    avatar: String,
    global_name: String,
) -> surrealdb::Result<()> {
    let db = database::connect().await.unwrap();

    let query = format!(
        "fn::join_user({}, '{}', '{}', {}, '{}', '{}')",
        id, username, joined_at, discord_id, avatar, global_name
    );
    dbg!(&query);
    let res = db.query(query).await?;
    dbg!(&res);
    Ok(())
}

pub async fn get_user(req: &HttpRequest) -> Result<String, String> {
    Ok("User".to_string())
}