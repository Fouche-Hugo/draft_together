<script setup lang="ts">
import PositionBottom from './icons/PositionBottom.vue';
import PositionJungle from './icons/PositionJungle.vue';
import PositionMid from './icons/PositionMid.vue';
import PositionSupport from './icons/PositionSupport.vue';
import PositionTop from './icons/PositionTop.vue';

defineEmits(["click"]);
</script>

<template>
  <div class="flex gap-4">
    <ChampionRole role="TOP" @click="$emit('click', 'TOP')"
      ><PositionTop
    /></ChampionRole>
    <ChampionRole role="JUNGLE" @click="$emit('click', 'JUNGLE')"
      ><PositionJungle
    /></ChampionRole>
    <ChampionRole role="MID" @click="$emit('click', 'MID')"
      ><PositionMid
    /></ChampionRole>
    <ChampionRole role="BOT" @click="$emit('click', 'BOT')"
      ><PositionBottom
    /></ChampionRole>
    <ChampionRole role="SUPPORT" @click="$emit('click', 'SUPPORT')"
      ><PositionSupport
    /></ChampionRole>
  </div>
</template>
