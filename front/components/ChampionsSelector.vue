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

function startDrag(event: DragEvent, championId: number) {
  if (event.dataTransfer !== null) {
    event.dataTransfer.dropEffect = "copy";
    event.dataTransfer.effectAllowed = "copy";
    event.dataTransfer.setData("championId", championId.toString());
    event.dataTransfer.setData("origin", JSON.stringify(null));
  }
}
const url = useRequestURL();
const imageBaseUrl = `${url.protocol}//${url.hostname}/`;
</script>

<template>
  <section
    class="grid grid-cols-4 gap-3 overflow-y-scroll border-zinc-600 sm:grid-cols-5 md:grid-cols-6 lg:grid-cols-7 xl:grid-cols-8 2xl:grid-cols-9"
  >
    <div
      v-for="champion in sorted_champions"
      :key="champion.id"
      draggable
      @dragstart="startDrag($event, champion.id)"
      @click="$emit('click', champion.id)"
    >
      <NuxtImg
        format="webp"
        :alt="champion.name"
        :src="`${imageBaseUrl}${champion.default_skin_image_path}`"
      />
    </div>
  </section>
</template>
