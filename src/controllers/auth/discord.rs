use crate::database::connect as database;

pub async fn discord_auth(access_token: String, expires_in: i64, scope: String) -> Result<(), String> {
    // @todo prio1 da inserire il token nel database
    println!("SONO PASSATO DI QUAAAAAA");
    Ok(())
}