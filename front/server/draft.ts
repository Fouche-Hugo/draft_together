export type ChampionIdsList = [number | null, number | null, number | null, number | null, number | null]

export interface Draft {
    blue_champions: ChampionIdsList;
    red_champions: ChampionIdsList;
    blue_bans: ChampionIdsList;
    red_bans: ChampionIdsList;
}
