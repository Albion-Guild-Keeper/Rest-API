use surrealdb::opt::auth::Record;
use crate::database::connect as database;
use crate::models::auth::sign_up::SignUp;

pub async fn sign_up(username: String, email: String, password: String) -> surrealdb::Result<()> {
    let db = database::connect().await.unwrap();

    let jwt = db
        .signup(Record {
            namespace: "root",
            database: "root",
            access: "accounts",
            params: SignUp {
                username,
                email,
                password,
            },
        })
        .await?;

    let token = jwt.as_insecure_token();
    dbg!(token);
    Ok(())
}