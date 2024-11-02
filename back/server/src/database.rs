use draft_together_data::{Champion, Draft};
use sqlx::{prelude::FromRow, query, query_as, PgPool};
use uuid::Uuid;

use crate::ServerDraft;

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
    pub blue_ban_1: Option<i32>,
    pub blue_ban_2: Option<i32>,
    pub blue_ban_3: Option<i32>,
    pub blue_ban_4: Option<i32>,
    pub blue_ban_5: Option<i32>,
    pub red_ban_1: Option<i32>,
    pub red_ban_2: Option<i32>,
    pub red_ban_3: Option<i32>,
    pub red_ban_4: Option<i32>,
    pub red_ban_5: Option<i32>,
    pub blue_1: Option<i32>,
    pub blue_2: Option<i32>,
    pub blue_3: Option<i32>,
    pub blue_4: Option<i32>,
    pub blue_5: Option<i32>,
    pub red_1: Option<i32>,
    pub red_2: Option<i32>,
    pub red_3: Option<i32>,
    pub red_4: Option<i32>,
    pub red_5: Option<i32>,
}

// impl DraftDatabase {
//     pub fn from_draft(draft: Draft, client_id: Uuid)
// }

impl From<DraftDatabase> for ServerDraft {
    fn from(value: DraftDatabase) -> Self {
        Self {
            id: value.id,
            draft: Draft {
                blue_champions: [
                    value.blue_1,
                    value.blue_2,
                    value.blue_3,
                    value.blue_4,
                    value.blue_5,
                ],
                red_champions: [
                    value.red_1,
                    value.red_2,
                    value.red_3,
                    value.red_4,
                    value.red_5,
                ],
                blue_bans: [
                    value.blue_ban_1,
                    value.blue_ban_2,
                    value.blue_ban_3,
                    value.blue_ban_4,
                    value.blue_ban_5,
                ],
                red_bans: [
                    value.red_ban_1,
                    value.red_ban_2,
                    value.red_ban_3,
                    value.red_ban_4,
                    value.red_ban_5,
                ],
            },
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

pub async fn update_champion(
    pool: &PgPool,
    champion: &ChampionDatabaseInsertion,
) -> Result<(), sqlx::Error> {
    query("UPDATE champion SET name = $1, default_skin_image_path = $2, centered_default_skin_image_path = $3 WHERE riot_id = $4")
        .bind(&champion.name)
        .bind(&champion.default_skin_image_path)
        .bind(&champion.centered_default_skin_image_path)
        .bind(&champion.riot_id)
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

pub async fn new_draft(pool: &PgPool, client_id: Uuid) -> Result<i32, sqlx::Error> {
    let row: (i32,) = query_as("INSERT INTO draft (client_id) VALUES ($1) RETURNING id")
        .bind(client_id)
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}

pub async fn draft_exists(pool: &PgPool, client_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = query("SELECT client_id FROM draft WHERE client_id = $1")
        .bind(client_id)
        .fetch_optional(pool)
        .await?;

    Ok(result.is_some())
}

pub async fn update_draft(pool: &PgPool, server_draft: &ServerDraft) -> Result<(), sqlx::Error> {
    let draft = &server_draft.draft;
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
    .bind(draft.blue_bans[0])
    .bind(draft.blue_bans[1])
    .bind(draft.blue_bans[2])
    .bind(draft.blue_bans[3])
    .bind(draft.blue_bans[4])
    .bind(draft.red_bans[0])
    .bind(draft.red_bans[1])
    .bind(draft.red_bans[2])
    .bind(draft.red_bans[3])
    .bind(draft.red_bans[4])
    .bind(draft.blue_champions[0])
    .bind(draft.blue_champions[1])
    .bind(draft.blue_champions[2])
    .bind(draft.blue_champions[3])
    .bind(draft.blue_champions[4])
    .bind(draft.red_champions[0])
    .bind(draft.red_champions[1])
    .bind(draft.red_champions[2])
    .bind(draft.red_champions[3])
    .bind(draft.red_champions[4])
    .bind(server_draft.id)
    .execute(pool)
    .await?;

    Ok(())
}
