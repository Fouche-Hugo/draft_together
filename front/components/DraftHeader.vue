<script setup lang="ts">
import type { ChampionsList } from "~/server/champion";
import { Team, type ChampionDropData } from "~/server/draft";
import BansTeam from "./BansTeam.vue";

interface Props {
  blueBans: ChampionsList;
  redBans: ChampionsList;
  link: string;
}

const props = defineProps<Props>();
defineEmits<{
  dblclick: [team: Team, index: number];
  drop: [championDropData: ChampionDropData];
}>();
</script>

<template>
  <header class="flex min-h-[100px] items-stretch justify-between">
    <BansTeam
      :champions="props.blueBans"
      :team="Team.Blue"
      @dblclick="(index) => $emit('dblclick', Team.Blue, index)"
      @drop="(championDropData) => $emit('drop', championDropData)"
    />
    <LinkButton :link />
    <BansTeam
      :champions="props.redBans"
      :team="Team.Red"
      @dblclick="(index) => $emit('dblclick', Team.Red, index)"
      @drop="(championDropData) => $emit('drop', championDropData)"
    />
  </header>
</template>
