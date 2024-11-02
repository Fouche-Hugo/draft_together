<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champion: Champion;
}

const props = defineProps<Props>();
</script>

<template>
  <div
    class="bg-cover bg-top"
    :style="`background-image: url(${props.champion.centered_default_skin_image_path})`"
  ></div>
</template>
