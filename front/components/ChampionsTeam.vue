<script setup lang="ts">
import type { ChampionsList } from "~/server/champion";
import ChampionTeam from "./ChampionTeam.vue";

interface Props {
  champions: ChampionsList;
  team: string;
}

const props = defineProps<Props>();
</script>

<template>
  <section class="grid grow grid-cols-1">
    <ChampionTeam
      v-for="(champion, index) in props.champions"
      :key="index"
      :champion
      :index
      :team
    />
  </section>
</template>
