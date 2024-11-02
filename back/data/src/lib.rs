use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Champion {
    pub id: ChampionId,
    pub riot_id: String,
    pub name: String,
    pub default_skin_image_path: String,
    pub centered_default_skin_image_path: String,
    pub positions: Vec<ChampionRole>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChampionRole {
    TOP,
    JUNGLE,
    MID,
    BOT,
    SUPPORT,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ChampionPosition {
    Blue1,
    Blue2,
    Blue3,
    Blue4,
    Blue5,
    Red1,
    Red2,
    Red3,
    Red4,
    Red5,
    BlueBan1,
    BlueBan2,
    BlueBan3,
    BlueBan4,
    BlueBan5,
    RedBan1,
    RedBan2,
    RedBan3,
    RedBan4,
    RedBan5,
}

pub type ChampionId = i32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChampionUpdate {
    pub champion_id: ChampionId,
    pub position: ChampionPosition,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub blue_champions: [Option<ChampionId>; 5],
    pub red_champions: [Option<ChampionId>; 5],
    pub blue_bans: [Option<ChampionId>; 5],
    pub red_bans: [Option<ChampionId>; 5],
}

impl Draft {
    pub fn display(&self) -> String {
        format!(
            "Blue bans: {:?}
            Red bans: {:?}
            Blue picks: {:?}
            Red picks: {:?}",
            self.blue_bans, self.red_bans, self.blue_champions, self.red_champions
        )
        .to_string()
    }

    pub fn update(&mut self, champion_update: &ChampionUpdate) {
        match champion_update.position {
            ChampionPosition::Blue1 => self.blue_champions[0] = Some(champion_update.champion_id),
            ChampionPosition::Blue2 => self.blue_champions[1] = Some(champion_update.champion_id),
            ChampionPosition::Blue3 => self.blue_champions[2] = Some(champion_update.champion_id),
            ChampionPosition::Blue4 => self.blue_champions[3] = Some(champion_update.champion_id),
            ChampionPosition::Blue5 => self.blue_champions[4] = Some(champion_update.champion_id),
            ChampionPosition::Red1 => self.red_champions[0] = Some(champion_update.champion_id),
            ChampionPosition::Red2 => self.red_champions[1] = Some(champion_update.champion_id),
            ChampionPosition::Red3 => self.red_champions[2] = Some(champion_update.champion_id),
            ChampionPosition::Red4 => self.red_champions[3] = Some(champion_update.champion_id),
            ChampionPosition::Red5 => self.red_champions[4] = Some(champion_update.champion_id),
            ChampionPosition::BlueBan1 => self.blue_bans[0] = Some(champion_update.champion_id),
            ChampionPosition::BlueBan2 => self.blue_bans[1] = Some(champion_update.champion_id),
            ChampionPosition::BlueBan3 => self.blue_bans[2] = Some(champion_update.champion_id),
            ChampionPosition::BlueBan4 => self.blue_bans[3] = Some(champion_update.champion_id),
            ChampionPosition::BlueBan5 => self.blue_bans[4] = Some(champion_update.champion_id),
            ChampionPosition::RedBan1 => self.red_bans[0] = Some(champion_update.champion_id),
            ChampionPosition::RedBan2 => self.red_bans[1] = Some(champion_update.champion_id),
            ChampionPosition::RedBan3 => self.red_bans[2] = Some(champion_update.champion_id),
            ChampionPosition::RedBan4 => self.red_bans[3] = Some(champion_update.champion_id),
            ChampionPosition::RedBan5 => self.red_bans[4] = Some(champion_update.champion_id),
        }
    }
}
