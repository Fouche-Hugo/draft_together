<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champions: Champion[];
}

const props = defineProps<Props>();
let sorted_champions: Champion[] = [];

watchEffect(() => {
  sorted_champions = props.champions.toSorted((a, b) => {
    if (a.name < b.name) {
      return -1;
    } else {
      return 1;
    }
  });
});
</script>

<template>
  <section
    class="grid max-w-3xl grid-cols-8 gap-3 overflow-scroll border-zinc-600"
  >
    <div v-for="champion in sorted_champions" :key="champion.id">
      <img :src="champion.default_skin_image_path" />
    </div>
  </section>
</template>
