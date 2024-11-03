<script setup lang="ts">
import type { Champion } from "~/server/champion";
import BansTeam from "./BansTeam.vue";

interface Props {
  blueBans: [Champion, Champion, Champion, Champion, Champion];
  redBans: [Champion, Champion, Champion, Champion, Champion];
  selectedIndex: [string, number] | null;
}

const props = defineProps<Props>();
defineEmits(["clickBlueBan", "clickRedBan"]);
</script>

<template>
  <header class="flex min-h-[100px] items-stretch justify-between">
    <BansTeam
      :champions="props.blueBans"
      :selected-index="
        selectedIndex?.[0] === 'BLUEBAN' ? selectedIndex[1] : null
      "
      @click="(index) => $emit('clickBlueBan', index)"
    />
    <BansTeam
      :champions="props.redBans"
      :selected-index="
        selectedIndex?.[0] === 'REDBAN' ? selectedIndex[1] : null
      "
      @click="(index) => $emit('clickRedBan', index)"
    />
  </header>
</template>
