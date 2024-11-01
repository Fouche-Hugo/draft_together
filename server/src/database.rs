use draft_together_data::{Champion, Draft};
use sqlx::{prelude::FromRow, query, query_as, PgPool};
use uuid::Uuid;

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

#[derive(Debug, FromRow)]
pub struct DraftDatabase {
    pub id: i32,
    pub client_id: Uuid,
    pub blue_ban_1: i32,
    pub blue_ban_2: i32,
    pub blue_ban_3: i32,
    pub blue_ban_4: i32,
    pub blue_ban_5: i32,
    pub red_ban_1: i32,
    pub red_ban_2: i32,
    pub red_ban_3: i32,
    pub red_ban_4: i32,
    pub red_ban_5: i32,
    pub blue_1: i32,
    pub blue_2: i32,
    pub blue_3: i32,
    pub blue_4: i32,
    pub blue_5: i32,
    pub red_1: i32,
    pub red_2: i32,
    pub red_3: i32,
    pub red_4: i32,
    pub red_5: i32,
}

impl From<DraftDatabase> for Draft {
    fn from(value: DraftDatabase) -> Self {
        Self {
            blue_champions: [
                Some(value.blue_1),
                Some(value.blue_2),
                Some(value.blue_3),
                Some(value.blue_4),
                Some(value.blue_5),
            ],
            red_champions: [
                Some(value.red_1),
                Some(value.red_2),
                Some(value.red_3),
                Some(value.red_4),
                Some(value.red_5),
            ],
            blue_bans: [
                Some(value.blue_ban_1),
                Some(value.blue_ban_2),
                Some(value.blue_ban_3),
                Some(value.blue_ban_4),
                Some(value.blue_ban_5),
            ],
            red_bans: [
                Some(value.red_ban_1),
                Some(value.red_ban_2),
                Some(value.red_ban_3),
                Some(value.red_ban_4),
                Some(value.red_ban_5),
            ],
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

pub async fn query_draft_by_client_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<DraftDatabase, sqlx::Error> {
    query_as("SELECT * FROM draft WHERE client_id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn new_draft(pool: &PgPool, client_id: Uuid) -> Result<(), sqlx::Error> {
    query("INSERT INTO draft (client_id) VALUES ($1)")
        .bind(client_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn draft_exists(pool: &PgPool, client_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = query("SELECT client_id FROM draft WHERE client_id = $1")
        .bind(client_id)
        .fetch_optional(pool)
        .await?;

    Ok(result.is_some())
}

pub async fn update_draft(pool: &PgPool, draft: &DraftDatabase) -> Result<(), sqlx::Error> {
    query(
        "UPDATE draft
        SET blue_ban_1 = $1,
        blue_ban_2 = $2,
        blue_ban_3 =  $3,
        blue_ban_4 = $4,
        blue_ban_5 = $5,
        red_ban_1 = $6,
        red_ban_2 = $7,
        red_ban_3 = $8,
        red_ban_4 = $9,
        red_ban_5 = $10,
        blue_1 = $11,
        blue_2 = $12,
        blue_3 = $13,
        blue_4 = $14,
        blue_5 = $15,
        red_1 = $16,
        red_2 = $17,
        red_3 = $18,
        red_4 = $19,
        red_5 = $20
        WHERE id = $21",
    )
    .bind(draft.blue_ban_1)
    .bind(draft.blue_ban_2)
    .bind(draft.blue_ban_3)
    .bind(draft.blue_ban_4)
    .bind(draft.blue_ban_5)
    .bind(draft.red_ban_1)
    .bind(draft.red_ban_2)
    .bind(draft.red_ban_3)
    .bind(draft.red_ban_4)
    .bind(draft.red_ban_5)
    .bind(draft.blue_1)
    .bind(draft.blue_2)
    .bind(draft.blue_3)
    .bind(draft.blue_4)
    .bind(draft.blue_5)
    .bind(draft.red_1)
    .bind(draft.red_2)
    .bind(draft.red_3)
    .bind(draft.red_4)
    .bind(draft.red_5)
    .execute(pool)
    .await?;

    Ok(())
}
