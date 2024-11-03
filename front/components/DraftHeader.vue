<script setup lang="ts">
import type { ChampionsList } from "~/server/champion";
import { Team } from "~/server/draft";
import BansTeam from "./BansTeam.vue";

interface Props {
  blueBans: ChampionsList;
  redBans: ChampionsList;
}

const props = defineProps<Props>();
</script>

<template>
  <header class="flex min-h-[100px] items-stretch justify-between">
    <BansTeam :champions="props.blueBans" :team="Team.Blue" />
    <BansTeam :champions="props.redBans" :team="Team.Red" />
  </header>
</template>
