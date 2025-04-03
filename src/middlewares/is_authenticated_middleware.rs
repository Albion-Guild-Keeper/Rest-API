use crate::utils::database::connect::connect;

pub async fn is_authenticated() -> Result<String, surrealdb::Error> {
    // Implement your authentication logic here
    // For example, check if a user is logged in or has a valid session

    let db = connect().await.unwrap();

    let response = db.query("SELECT * FROM cat").await.unwrap();

    Ok(format!("{:#?}", response))
}
