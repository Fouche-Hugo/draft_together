export type ChampionIdsList = [number | null, number | null, number | null, number | null, number | null]

export interface Draft {
    blue_champions: ChampionIdsList;
    red_champions: ChampionIdsList;
    blue_bans: ChampionIdsList;
    red_bans: ChampionIdsList;
}

export interface DraftUpdate {
    champion_id: number,
    position: string
}

export enum Team {
    Blue = "Blue",
    Red = "Red",
}

export interface Selection {
    team: Team,
    isBan: boolean,
    index: number
}

export function computePosition(team: Team, index: number, isBan = false): string {
    const index_server = index + 1;
    return isBan ? `${team}Ban${index_server}` : `${team}${index_server}`
}