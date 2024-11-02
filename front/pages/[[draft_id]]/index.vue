<script setup lang="ts">
import ChampionsTeam from "~/components/ChampionsTeam.vue";
import { validate } from "uuid";
import ChampionsSelector from "~/components/ChampionsSelector.vue";
import type { Champion } from "~/server/champion";
import DraftHeader from "~/components/DraftHeader.vue";
import DraftFooter from "~/components/DraftFooter.vue";

const { data } = await useFetch<Champion[]>("http://app:3000/champions");
const champions = data;

definePageMeta({
  validate: async (route) => {
    console.log(route.params.draft_id);
    return validate(route.params.draft_id);
  },
});
</script>

<template>
  <div class="flex h-full flex-col items-stretch gap-4">
    <DraftHeader />
    <main class="flex overflow-scroll">
      <ChampionsTeam
        v-if="champions !== null"
        :champions="[
          champions[0],
          champions[1],
          champions[2],
          champions[3],
          champions[4],
        ]"
      />
      <ChampionsSelector v-if="champions !== null" :champions />
      <ChampionsTeam
        v-if="champions !== null"
        :champions="[
          champions[0],
          champions[1],
          champions[2],
          champions[3],
          champions[4],
        ]"
      />
    </main>
    <DraftFooter />
  </div>
</template>
