<script setup lang="ts">
import type { ChampionsList } from "~/server/champion";
import ChampionTeam from "./ChampionTeam.vue";
import type { ChampionDropData, Team } from "~/server/draft";

interface Props {
  champions: ChampionsList;
  team: Team;
}

const props = defineProps<Props>();
defineEmits<{
  dblclick: [index: number];
  drop: [data: ChampionDropData];
}>();
</script>

<template>
  <section class="grid grow grid-cols-1 h-full">
    <ChampionTeam
      v-for="(champion, index) in props.champions"
      :key="index"
      :champion
      :index
      :team
      @dblclick="$emit('dblclick', index)"
      @drop="
        (newChampionId, currentChampionId, origin) =>
          $emit('drop', {
            newChampionId,
            currentChampionId,
            newPosition: { team, index, isBan: false },
            origin,
          })
      "
    />
  </section>
</template>
