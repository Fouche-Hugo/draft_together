<script setup lang="ts">
import type { Champion } from "~/server/champion";
import BansTeam from "./BansTeam.vue";

interface Props {
  blueBans: [
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
  ];
  redBans: [
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
  ];
}

const props = defineProps<Props>();
</script>

<template>
  <header class="flex min-h-[100px] items-stretch justify-between">
    <BansTeam :champions="props.blueBans" team="BLUE" />
    <BansTeam :champions="props.redBans" team="RED" />
  </header>
</template>
