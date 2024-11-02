<script setup lang="ts">
import ChampionsTeam from "~/components/ChampionsTeam.vue";
import { validate } from "uuid";
import ChampionsSelector from "~/components/ChampionsSelector.vue";
import type { Champion } from "~/server/champion";
import DraftHeader from "~/components/DraftHeader.vue";
import DraftFooter from "~/components/DraftFooter.vue";
import ChampionRole from "~/components/ChampionRole.vue";
import PositionTop from "~/components/icons/PositionTop.vue";
import PositionJungle from "~/components/icons/PositionJungle.vue";
import PositionMid from "~/components/icons/PositionMid.vue";
import PositionBottom from "~/components/icons/PositionBottom.vue";
import PositionSupport from "~/components/icons/PositionSupport.vue";
import SearchInput from "~/components/SearchInput.vue";

const { data } = await useFetch<Champion[]>("http://app:3000/champions");
const champions = data.value !== null ? data.value : [];

definePageMeta({
  validate: async (route) => {
    console.log(route.params.draft_id);
    return validate(route.params.draft_id);
  },
});

let roleSelected: string | null = null;
const searchInput = ref("");

const filtered_champions = ref(champions);
function filter_champions(role: string) {
  filtered_champions.value = champions.filter((champion) => {
    return champion.positions.includes(role);
  });
  roleSelected = role;
}

function unfilter_champions() {
  filtered_champions.value = champions;
  roleSelected = null;
}
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
      />
      <div class="flex w-2/5 flex-col items-stretch gap-4 overflow-scroll px-4">
        <div class="flex justify-between gap-4">
          <div class="flex gap-4">
            <ChampionRole
              :is-selected="roleSelected === 'TOP'"
              @click="
                roleSelected === 'TOP'
                  ? unfilter_champions()
                  : filter_champions('TOP')
              "
              ><PositionTop
            /></ChampionRole>
            <ChampionRole
              :is-selected="roleSelected === 'JUNGLE'"
              @click="
                roleSelected === 'JUNGLE'
                  ? unfilter_champions()
                  : filter_champions('JUNGLE')
              "
              ><PositionJungle
            /></ChampionRole>
            <ChampionRole
              :is-selected="roleSelected === 'MID'"
              @click="
                roleSelected === 'MID'
                  ? unfilter_champions()
                  : filter_champions('MID')
              "
              ><PositionMid
            /></ChampionRole>
            <ChampionRole
              :is-selected="roleSelected === 'BOT'"
              @click="
                roleSelected === 'BOT'
                  ? unfilter_champions()
                  : filter_champions('BOT')
              "
              ><PositionBottom
            /></ChampionRole>
            <ChampionRole
              :is-selected="roleSelected === 'SUPPORT'"
              @click="
                roleSelected === 'SUPPORT'
                  ? unfilter_champions()
                  : filter_champions('SUPPORT')
              "
              ><PositionSupport
            /></ChampionRole>
          </div>
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
      />
    </main>
    <DraftFooter />
  </div>
</template>
