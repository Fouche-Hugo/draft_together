<script setup lang="ts">
import type { Champion } from "~/server/champion";
import ChampionTeam from "./ChampionTeam.vue";

interface Props {
  champions: [Champion, Champion, Champion, Champion, Champion];
  selectedIndex: number | null;
}

const props = defineProps<Props>();
defineEmits(["click"])
</script>

<template>
  <section class="grid grow grid-cols-1">
    <ChampionTeam
      v-for="(champion, index) in props.champions"
      :key="champion.id"
      :champion
      :is-selected="index === selectedIndex"
      @click="$emit('click', index)"
    />
  </section>
</template>
