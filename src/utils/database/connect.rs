use surrealdb::engine::any;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;

pub async fn connect() -> Result<Surreal<any::Any>, Box<dyn std::error::Error>> {
    let db = any::connect("wss://magnetic-rail-06akp389stprh9se8brkar7b1k.aws-euw1.surreal.cloud")
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to the database: {}", e);
            e
        })?;

    // Select a namespace and database
    db.use_ns("root").use_db("root").await.map_err(|e| {
        eprintln!("Failed to select namespace and database: {}", e);
        e
    })?;

    // Authenticate
    db.signin(Root {
        username: "root",
        password: "root",
    })
        .await
        .map_err(|e| {
            eprintln!("Failed to authenticate: {}", e);
            e
        })?;

    Ok(db)
}
