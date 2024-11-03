<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champions: [
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
  ];
  team: string;
}

const props = defineProps<Props>();
const selection: Ref<[string, number] | null> = inject("selection")!;
</script>

<template>
  <div class="grid h-full grid-cols-5">
    <template v-for="(champion, index) in props.champions" :key="champion.id">
      <button
        v-if="champion !== null"
        class="relative h-24 w-24 bg-cover"
        :style="`background-image: url(${champion.default_skin_image_path})`"
        @click="
          selection =
            selection?.[0] !== `BAN${team}` || selection[1] !== index
              ? [`BAN${team}`, index]
              : null
        "
      >
        <div
          class="absolute inset-0 border-zinc-100"
          :class="{
            border: selection?.[0] === `BAN${team}` && selection?.[1] === index,
          }"
        ></div>
      </button>
      <button
        v-else
        class="relative h-24 w-24 bg-cover"
        :style="`background-image: url(${champion.default_skin_image_path})`"
        @click="
          selection =
            selection?.[0] !== `BAN${team}` || selection[1] !== index
              ? [`BAN${team}`, index]
              : null
        "
      >
        <div
          class="absolute inset-0 border-zinc-100"
          :class="{
            border: selection?.[0] === `BAN${team}` && selection?.[1] === index,
          }"
        ></div>
      </button>
    </template>
  </div>
</template>
