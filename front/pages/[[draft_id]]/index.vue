<script setup lang="ts">
import ChampionsTeam from "~/components/ChampionsTeam.vue";
import { validate } from "uuid";
import ChampionsSelector from "~/components/ChampionsSelector.vue";
import type { ChampionIdsList, Draft } from "~/server/draft";
import type { Champion, ChampionsList } from "~/server/champion";
import DraftHeader from "~/components/DraftHeader.vue";
import DraftFooter from "~/components/DraftFooter.vue";
import SearchInput from "~/components/SearchInput.vue";
import ChampionRoles from "~/components/ChampionRoles.vue";

definePageMeta({
  validate: async (route) => {
    return validate(route.params.draft_id);
  },
});

const route = useRoute();
const { data: champions_fetched } = await useFetch<Champion[]>(
  "http://app:3000/champions",
);
const champions =
  champions_fetched.value !== null ? champions_fetched.value : [];

const { data: draft_fetched } = await useFetch<Draft>(
  `http://app:3000/draft/${route.params.draft_id}`,
);
const draft: Ref<Draft> =
  draft_fetched.value !== null
    ? ref(draft_fetched.value)
    : ref({
        blue_champions: [null, null, null, null, null],
        red_champions: [null, null, null, null, null],
        blue_bans: [null, null, null, null, null],
        red_bans: [null, null, null, null, null],
      });

if (import.meta.client) {
  const webSocket = new WebSocket(
    `ws://localhost:3636/ws/${route.params.draft_id}`,
  );
  webSocket.onmessage = (event: MessageEvent<string>) => {
    draft.value = JSON.parse(event.data);
  };
  webSocket.onerror = (error) => console.log("ws error: ", error);
}

function mapChampions(indexes: ChampionIdsList): ChampionsList {
  return indexes.map((id) => {
    const championIndex = champions.find((champion) => {
      return champion.id === id;
    });
    return championIndex !== undefined ? championIndex : null;
  }) as ChampionsList;
}

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
      :blue-bans="mapChampions(draft.blue_bans)"
      :red-bans="mapChampions(draft.red_bans)"
    />
    <main class="flex grow items-stretch overflow-scroll">
      <ChampionsTeam
        :champions="mapChampions(draft.blue_champions)"
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
        :champions="mapChampions(draft.red_champions)"
        :selected-index="selection?.[0] === 'RED' ? selection[1] : null"
        team="RED"
      />
    </main>
    <DraftFooter />
  </div>
</template>
