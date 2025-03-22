use crate::database::connect as database;
use crate::models::discord::Discord;
use crate::utils::surreal_int::SurrealInt;


pub async fn join_discord(
    id: SurrealInt,
    discord_name: String,
    joined_at: String
) -> Result<Option<Discord>, Box<dyn std::error::Error>> {

    let db = database::connect().await.unwrap(); 
    // Check if the discord_id already exists
    let existing_discord: Option<Discord> = db.select(("discords", id.to_string())).await?;

    match existing_discord {
        Some(mut existing) => {
            // Discord ID exists, update only the discord_name
            existing.discord_name = discord_name;
            let updated_discord: Option<Discord> = db.update(("discords", id.to_string())).content(existing).await?;
            Ok(updated_discord)
        }
        None => {
            // Discord ID does not exist, create a new record
            let discord = Discord {
                id,
                discord_name,
                joined_at,
            };

            let created_discord: Option<Discord> = db.create("discords")
                .content(discord)
                .await?;
            
            Ok(created_discord)
        }
    }
}