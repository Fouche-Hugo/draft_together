use sqlx::{prelude::FromRow, query, query_as, PgPool};

#[derive(Debug, FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct ChampionDatabase {
    id: i64,
    riot_id: String,
    name: String,
    default_skin_image_path: String,
    centered_default_skin_image_path: String,
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
        "SELECT id, riotId, name, defaultSkinImagePath, centeredDefaultSkinImagePath FROM champion",
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_champion(
    pool: &PgPool,
    champion: &ChampionDatabaseInsertion,
) -> Result<(), sqlx::Error> {
    query("INSERT INTO champion (riotId, name, defaultSkinImagePath, centeredDefaultSkinImagePath) VALUES ($1, $2, $3, $4)")
        .bind(&champion.riot_id)
        .bind(&champion.name)
        .bind(&champion.default_skin_image_path)
        .bind(&champion.centered_default_skin_image_path)
        .execute(pool).await?;

    Ok(())
}

pub async fn champion_exists(pool: &PgPool, riot_id: &str) -> Result<bool, sqlx::Error> {
    let result = query("SELECT riotId FROM champion WHERE riotId = $1")
        .bind(riot_id)
        .fetch_optional(pool)
        .await?;

    Ok(result.is_some())
}
