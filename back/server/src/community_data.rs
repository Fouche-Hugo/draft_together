use std::collections::HashMap;

use anyhow::Result;
use draft_together_data::ChampionRole;
use serde::Deserialize;
use tracing::{debug, trace};

type CommunityChampionId = i32;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PositionRate {
    pub play_rate: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct ChampionRates {
    pub top: PositionRate,
    pub jungle: PositionRate,
    pub middle: PositionRate,
    pub bottom: PositionRate,
    pub utility: PositionRate,
}

const PLAY_RATE_THRESHOLD: f32 = 0.1;
impl From<ChampionRates> for Vec<ChampionRole> {
    fn from(value: ChampionRates) -> Self {
        let mut roles = Vec::with_capacity(5);

        if value.top.play_rate > PLAY_RATE_THRESHOLD {
            roles.push(ChampionRole::TOP);
        }
        if value.jungle.play_rate > PLAY_RATE_THRESHOLD {
            roles.push(ChampionRole::JUNGLE);
        }
        if value.middle.play_rate > PLAY_RATE_THRESHOLD {
            roles.push(ChampionRole::MID);
        }
        if value.bottom.play_rate > PLAY_RATE_THRESHOLD {
            roles.push(ChampionRole::BOT);
        }
        if value.utility.play_rate > PLAY_RATE_THRESHOLD {
            roles.push(ChampionRole::SUPPORT);
        }

        roles
    }
}

#[derive(Debug, Deserialize)]
pub struct RatesData {
    pub data: HashMap<CommunityChampionId, ChampionRates>,
}

#[derive(Debug, Deserialize)]
pub struct CommunityChampion {
    pub id: i32,
    pub name: String,
    pub alias: String,
}

pub async fn get_champions_rates() -> Result<RatesData> {
    debug!("fetching merakianalytics to get champions rates");
    let rates: RatesData = reqwest::get(
        "http://cdn.merakianalytics.com/riot/lol/resources/latest/en-US/championrates.json",
    )
    .await?
    .json()
    .await?;

    trace!("champions rates: {rates:?}");

    Ok(rates)
}

pub async fn get_community_champion_ids() -> Result<Vec<CommunityChampion>> {
    debug!("fetching community dragon to get champions ids");

    let community_champions: Vec<CommunityChampion> = reqwest::get(
        "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json"
    ).await?
    .json()
    .await?;

    trace!("community champions: {community_champions:?}");

    Ok(community_champions)
}
