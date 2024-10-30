use draft_together_data::Champion;
use sqlx::{prelude::FromRow, query, query_as, PgPool};

#[derive(Debug, FromRow)]
pub struct ChampionDatabase {
    pub id: i32,
    pub riot_id: String,
    pub name: String,
    pub default_skin_image_path: String,
    pub centered_default_skin_image_path: String,
}

impl From<ChampionDatabase> for Champion {
    fn from(value: ChampionDatabase) -> Self {
        Self {
            id: value.id,
            riot_id: value.riot_id,
            name: value.name,
            default_skin_image_path: value.default_skin_image_path,
            centered_default_skin_image_path: value.centered_default_skin_image_path,
        }
    }
}

#[derive(Debug)]
pub struct ChampionDatabaseInsertion {
    pub riot_id: String,
    pub name: String,
    pub default_skin_image_path: String,
    pub centered_default_skin_image_path: String,
}

pub async fn query_champions(pool: &PgPool) -> Result<Vec<ChampionDatabase>, sqlx::Error> {
    query_as(
        "SELECT id, riot_id, name, default_skin_image_path, centered_default_skin_image_path FROM champion",
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_champion(
    pool: &PgPool,
    champion: &ChampionDatabaseInsertion,
) -> Result<(), sqlx::Error> {
    query("INSERT INTO champion (riot_id, name, default_skin_image_path, centered_default_skin_image_path) VALUES ($1, $2, $3, $4)")
        .bind(&champion.riot_id)
        .bind(&champion.name)
        .bind(&champion.default_skin_image_path)
        .bind(&champion.centered_default_skin_image_path)
        .execute(pool).await?;

    Ok(())
}

pub async fn champion_exists(pool: &PgPool, riot_id: &str) -> Result<bool, sqlx::Error> {
    let result = query("SELECT riot_id FROM champion WHERE riot_id = $1")
        .bind(riot_id)
        .fetch_optional(pool)
        .await?;

    Ok(result.is_some())
}
