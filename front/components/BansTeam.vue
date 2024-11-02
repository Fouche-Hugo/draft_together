<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champions: [Champion, Champion, Champion, Champion, Champion];
}

const props = defineProps<Props>();
</script>

<template>
  <div class="grid h-full grid-cols-5">
    <div
      v-for="champion in props.champions"
      :key="champion.id"
      class="bg-cover w-24 h-24"
      :style="`background-image: url(${champion.default_skin_image_path})`"
    ></div>
  </div>
</template>
