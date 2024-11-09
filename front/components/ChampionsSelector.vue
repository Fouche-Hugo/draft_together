<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champions: Champion[];
  searchInput: string;
}

const props = defineProps<Props>();
defineEmits(["click"]);

let sorted_champions: Champion[] = [];

watchEffect(() => {
  sorted_champions = props.champions
    .toSorted((a, b) => {
      if (a.name < b.name) {
        return -1;
      } else {
        return 1;
      }
    })
    .filter((champion) => {
      return champion.name
        .toLowerCase()
        .includes(props.searchInput.toLowerCase());
    });
});
</script>

<template>
  <section class="grid grid-cols-8 gap-3 overflow-y-scroll border-zinc-600">
    <div
      v-for="champion in sorted_champions"
      :key="champion.id"
      @click="$emit('click', champion.id)"
    >
      <img :src="champion.default_skin_image_path" />
    </div>
  </section>
</template>
