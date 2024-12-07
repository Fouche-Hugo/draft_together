<script setup lang="ts">
import ChampionsTeam from "~/components/ChampionsTeam.vue";
import { validate } from "uuid";
import ChampionsSelector from "~/components/ChampionsSelector.vue";
import {
  computePosition,
  Team,
  type ChampionDropData,
  type ChampionIdsList,
  type Draft,
  type Selection,
} from "~/server/draft";
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

const url = useRequestURL();

const runtimeConfig = useRuntimeConfig();
const route = useRoute();
const { data: champions_fetched } = await useFetch<Champion[]>(
  `${runtimeConfig.public.httpBaseAddress}champions`,
);
const champions =
  champions_fetched.value !== null ? champions_fetched.value : [];

const { data: draft_fetched } = await useFetch<Draft>(
  `${runtimeConfig.public.httpBaseAddress}draft/${route.params.draft_id}`,
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

let webSocket: WebSocket;
function wsConnect() {
  webSocket = new WebSocket(
    `${runtimeConfig.public.wsBaseAddress}${route.params.draft_id}`,
  );
  webSocket.onmessage = (event: MessageEvent<string>) => {
    draft.value = JSON.parse(event.data);
  };
  webSocket.onerror = (error) => {
    console.log("ws error: ", error);
    webSocket.close();
  };
  webSocket.onclose = () => {
    setTimeout(wsConnect, 1000);
  };
}

if (import.meta.client) {
  wsConnect();
}

function sendDraftUpdateClick(championId: number) {
  if (selection.value !== null) {
    webSocket.send(
      JSON.stringify({
        champion_id: championId,
        position: computePosition(
          selection.value.team,
          selection.value.index,
          selection.value.isBan,
        ),
      }),
    );
  }
}

function sendDraftUpdateDrop(championDropData: ChampionDropData) {
  webSocket.send(
    JSON.stringify({
      champion_id: championDropData.newChampionId,
      position: computePosition(
        championDropData.newPosition.team,
        championDropData.newPosition.index,
        championDropData.newPosition.isBan,
      ),
    }),
  );

  if (championDropData.origin !== null) {
    webSocket.send(
      JSON.stringify({
        champion_id: championDropData.currentChampionId,
        position: computePosition(
          championDropData.origin.team,
          championDropData.origin.index,
          championDropData.origin.isBan,
        ),
      }),
    );
  }
}

function sendDraftChampionReset(team: Team, index: number, isBan: boolean) {
  webSocket.send(
    JSON.stringify({
      champion_id: null,
      position: computePosition(team, index, isBan),
    }),
  );
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

const selection: Ref<Selection | null> = ref(null);
provide("selection", selection);
</script>

<template>
  <div id="draft" class="flex h-full flex-col items-stretch gap-4">
    <DraftHeader
      :blue-bans="mapChampions(draft.blue_bans)"
      :red-bans="mapChampions(draft.red_bans)"
      :link="url.toString()"
      imageHtmlElementId="draft"
      @dblclick="(team, index) => sendDraftChampionReset(team, index, true)"
      @drop="(championDropData) => sendDraftUpdateDrop(championDropData)"
    />
    <main class="flex grow items-stretch overflow-hidden">
      <ChampionsTeam
        :champions="mapChampions(draft.blue_champions)"
        :team="Team.Blue"
        @dblclick="(index) => sendDraftChampionReset(Team.Blue, index, false)"
        @drop="(championDropData) => sendDraftUpdateDrop(championDropData)"
      />
      <div
        class="flex w-[280px] flex-col items-stretch gap-4 overflow-hidden px-4 sm:w-[400px] md:w-[500px] lg:w-[600px] xl:w-[650px] 2xl:w-[800px]"
      >
        <div class="flex flex-col justify-between gap-4 sm:flex-row">
          <ChampionRoles
            class="self-center"
            @click="(role) => filter_champions(role)"
          />
          <SearchInput v-model="searchInput" />
        </div>
        <ChampionsSelector
          :champions="filtered_champions"
          :search-input="searchInput"
          @click="(id) => sendDraftUpdateClick(id)"
        />
      </div>
      <ChampionsTeam
        :champions="mapChampions(draft.red_champions)"
        :team="Team.Red"
        @dblclick="(index) => sendDraftChampionReset(Team.Red, index, false)"
        @drop="(championDropData) => sendDraftUpdateDrop(championDropData)"
      />
    </main>
    <DraftFooter />
  </div>
</template>
