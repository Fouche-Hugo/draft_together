<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champion: Champion;
  isSelected: boolean;
}

const props = defineProps<Props>();
defineEmits(["click"]);
</script>

<template>
  <button
    class="relative bg-cover bg-top"
    :style="`background-image: url(${props.champion.centered_default_skin_image_path})`"
    @click="$emit('click')"
  >
    <div
      class="absolute inset-0 border-zinc-100"
      :class="{ border: isSelected }"
    ></div>
  </button>
</template>
