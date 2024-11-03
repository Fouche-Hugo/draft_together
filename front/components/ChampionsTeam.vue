<script setup lang="ts">
import type { Champion } from "~/server/champion";
import ChampionTeam from "./ChampionTeam.vue";

interface Props {
  champions: [Champion, Champion, Champion, Champion, Champion];
  team: string;
}

const props = defineProps<Props>();
</script>

<template>
  <section class="grid grow grid-cols-1">
    <ChampionTeam
      v-for="(champion, index) in props.champions"
      :key="champion.id"
      :champion
      :index
      :team
    />
  </section>
</template>
