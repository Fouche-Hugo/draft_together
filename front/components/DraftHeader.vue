<script setup lang="ts">
import type { ChampionsList } from "~/server/champion";
import { Team } from "~/server/draft";
import BansTeam from "./BansTeam.vue";

interface Props {
  blueBans: ChampionsList;
  redBans: ChampionsList;
}

const props = defineProps<Props>();
defineEmits(["dblclick"]);
</script>

<template>
  <header class="flex min-h-[100px] items-stretch justify-between">
    <BansTeam
      :champions="props.blueBans"
      :team="Team.Blue"
      @dbl-click="(index) => $emit('dblclick', Team.Blue, index)"
    />
    <BansTeam
      :champions="props.redBans"
      :team="Team.Red"
      @dbl-click="(index) => $emit('dblclick', Team.Red, index)"
    />
  </header>
</template>
