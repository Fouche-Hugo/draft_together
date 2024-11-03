<script setup lang="ts">
import type { Champion } from "~/server/champion";

interface Props {
  champion: Champion | null;
  team: string;
  index: number;
}

const props = defineProps<Props>();

const selection: Ref<[string, number] | null> = inject("selection")!;
</script>

<template>
  <button
    v-if="props.champion !== null"
    class="relative bg-cover bg-top"
    :style="`background-image: url(${props.champion.centered_default_skin_image_path})`"
    @click="
      selection =
        selection?.[0] !== `${team}` || selection[1] !== index
          ? [`${team}`, index]
          : null
    "
  >
    <div
      class="absolute inset-0 border-zinc-100"
      :class="{
        border: selection?.[0] === `${team}` && selection?.[1] === index,
      }"
    ></div>
  </button>
  <button
    v-else
    class="relative bg-cover bg-top"
    @click="
      selection =
        selection?.[0] !== `${team}` || selection[1] !== index
          ? [`${team}`, index]
          : null
    "
  >
    <div
      class="absolute inset-0 border"
      :class="{
        'border-zinc-100':
          selection?.[0] === `${team}` && selection?.[1] === index,
        'border-zinc-600':
          selection?.[0] !== `${team}` || selection?.[1] !== index,
      }"
    ></div>
  </button>
</template>
