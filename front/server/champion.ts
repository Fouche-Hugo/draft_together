export interface Champion {
  id: number;
  riot_id: string;
  name: string;
  default_skin_image_path: string;
  centered_default_skin_image_path: string;
  positions: string[]
}

export type ChampionsList = [Champion | null, Champion | null, Champion | null, Champion | null, Champion | null]