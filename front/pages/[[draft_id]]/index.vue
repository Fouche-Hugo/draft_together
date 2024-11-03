<script setup lang="ts">
import ChampionsTeam from "~/components/ChampionsTeam.vue";
import { validate } from "uuid";
import ChampionsSelector from "~/components/ChampionsSelector.vue";
import type { Champion } from "~/server/champion";
import DraftHeader from "~/components/DraftHeader.vue";
import DraftFooter from "~/components/DraftFooter.vue";
import SearchInput from "~/components/SearchInput.vue";
import ChampionRoles from "~/components/ChampionRoles.vue";

const { data } = await useFetch<Champion[]>("http://app:3000/champions");
const champions = data.value !== null ? data.value : [];

definePageMeta({
  validate: async (route) => {
    console.log(route.params.draft_id);
    return validate(route.params.draft_id);
  },
});

const roleSelected: Ref<string | null> = ref(null);
provide("roleSelected", roleSelected);
const searchInput = ref("");

const filtered_champions = ref(champions);
function filter_champions(role: string) {
  if (role === roleSelected.value) {
    unfilter_champions();
  } else {
    filtered_champions.value = champions.filter((champion) => {
      return champion.positions.includes(role);
    });
    roleSelected.value = role;
  }
}

function unfilter_champions() {
  filtered_champions.value = champions;
  roleSelected.value = null;
}

const selection: Ref<[string, number] | null> = ref(null);
provide("selection", selection);
</script>

<template>
  <div class="flex h-full flex-col items-stretch gap-4">
    <DraftHeader
      v-if="champions !== null"
      :blue-bans="[
        champions[0],
        champions[1],
        champions[2],
        champions[3],
        champions[4],
      ]"
      :red-bans="[
        champions[0],
        champions[1],
        champions[2],
        champions[3],
        champions[4],
      ]"
    />
    <main class="flex grow items-stretch overflow-scroll">
      <ChampionsTeam
        :champions="[
          champions[0],
          champions[1],
          champions[2],
          champions[3],
          champions[4],
        ]"
        :selected-index="selection?.[0] === 'BLUE' ? selection[1] : null"
        team="BLUE"
      />
      <div class="flex w-2/5 flex-col items-stretch gap-4 overflow-scroll px-4">
        <div class="flex justify-between gap-4">
          <ChampionRoles @click="(role) => filter_champions(role)" />
          <SearchInput v-model="searchInput" />
        </div>
        <ChampionsSelector
          :champions="filtered_champions"
          :search-input="searchInput"
        />
      </div>
      <ChampionsTeam
        :champions="[
          champions[0],
          champions[1],
          champions[2],
          champions[3],
          champions[4],
        ]"
        :selected-index="selection?.[0] === 'RED' ? selection[1] : null"
        team="RED"
      />
    </main>
    <DraftFooter />
  </div>
</template>
