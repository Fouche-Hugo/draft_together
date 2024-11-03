<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champions: [Champion, Champion, Champion, Champion, Champion];
  selectedIndex: number | null;
}

const props = defineProps<Props>();

defineEmits(["click"]);
</script>

<template>
  <div class="grid h-full grid-cols-5">
    <button
      v-for="(champion, index) in props.champions"
      :key="champion.id"
      class="relative h-24 w-24 bg-cover"
      :style="`background-image: url(${champion.default_skin_image_path})`"
      @click="$emit('click', index)"
    >
      <div
        class="absolute inset-0 border-zinc-100"
        :class="{ border: selectedIndex === index }"
      ></div>
    </button>
  </div>
</template>
