use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Champion {
    pub name: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChampionUpdate {
    pub champion: Champion,
    pub position: ChampionPosition,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub blue_champions: [Option<Champion>; 5],
    pub red_champions: [Option<Champion>; 5],
    pub blue_bans: [Option<Champion>; 5],
    pub red_bans: [Option<Champion>; 5],
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

    pub fn update(&mut self, champion_update: ChampionUpdate) {
        match champion_update.position {
            ChampionPosition::Blue1 => self.blue_champions[0] = Some(champion_update.champion),
            ChampionPosition::Blue2 => self.blue_champions[1] = Some(champion_update.champion),
            ChampionPosition::Blue3 => self.blue_champions[2] = Some(champion_update.champion),
            ChampionPosition::Blue4 => self.blue_champions[3] = Some(champion_update.champion),
            ChampionPosition::Blue5 => self.blue_champions[4] = Some(champion_update.champion),
            ChampionPosition::Red1 => self.red_champions[0] = Some(champion_update.champion),
            ChampionPosition::Red2 => self.red_champions[1] = Some(champion_update.champion),
            ChampionPosition::Red3 => self.red_champions[2] = Some(champion_update.champion),
            ChampionPosition::Red4 => self.red_champions[3] = Some(champion_update.champion),
            ChampionPosition::Red5 => self.red_champions[4] = Some(champion_update.champion),
            ChampionPosition::BlueBan1 => self.blue_bans[0] = Some(champion_update.champion),
            ChampionPosition::BlueBan2 => self.blue_bans[1] = Some(champion_update.champion),
            ChampionPosition::BlueBan3 => self.blue_bans[2] = Some(champion_update.champion),
            ChampionPosition::BlueBan4 => self.blue_bans[3] = Some(champion_update.champion),
            ChampionPosition::BlueBan5 => self.blue_bans[4] = Some(champion_update.champion),
            ChampionPosition::RedBan1 => self.red_bans[0] = Some(champion_update.champion),
            ChampionPosition::RedBan2 => self.red_bans[1] = Some(champion_update.champion),
            ChampionPosition::RedBan3 => self.red_bans[2] = Some(champion_update.champion),
            ChampionPosition::RedBan4 => self.red_bans[3] = Some(champion_update.champion),
            ChampionPosition::RedBan5 => self.red_bans[4] = Some(champion_update.champion),
        }
    }
}
