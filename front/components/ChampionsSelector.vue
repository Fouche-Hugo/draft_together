<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champions: Champion[];
}

const props = defineProps<Props>();
</script>

<template>
  <section
    class="grid max-w-3xl grid-cols-8 gap-3 overflow-scroll border-zinc-600 px-4"
  >
    <div v-for="champion in props.champions" :key="champion.id">
      <img :src="champion.default_skin_image_path" />
    </div>
  </section>
</template>
