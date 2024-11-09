<script setup lang="ts">
import type { ChampionsList } from "~/server/champion";
import ChampionTeam from "./ChampionTeam.vue";
import type { Team } from "~/server/draft";

interface Props {
  champions: ChampionsList;
  team: Team;
}

const props = defineProps<Props>();
defineEmits(["dblclick"]);
</script>

<template>
  <section class="grid grow grid-cols-1">
    <ChampionTeam
      v-for="(champion, index) in props.champions"
      :key="index"
      :champion
      :index
      :team
      @dblclick="$emit('dblclick', index)"
    />
  </section>
</template>
