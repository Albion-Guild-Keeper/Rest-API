use surrealdb::opt::auth::Record;
use crate::database::connect as database;
use crate::models::auth::sign_in::SignIn;

pub async fn sign_in(email: String, password: String) -> surrealdb::Result<()> {
    let db = database::connect().await.unwrap();

    let jwt = db
        .signin(Record {
            namespace: "root",
            database: "root",
            access: "accounts",
            params: SignIn {
                email,
                password,
            },
        })
        .await?;

    let token = jwt.as_insecure_token();
    dbg!(token);
    Ok(())
}