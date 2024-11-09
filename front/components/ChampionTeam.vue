<script setup lang="ts">
import type { Champion } from "~/server/champion";
import type { Selection, Team } from "~/server/draft";

interface Props {
  champion: Champion | null;
  team: Team;
  index: number;
}

const props = defineProps<Props>();

const selection: Ref<Selection | null> = inject("selection")!;

function updateSelection() {
  if (
    selection.value?.team !== props.team ||
    selection.value.isBan ||
    selection.value.index !== props.index
  ) {
    selection.value = {
      team: props.team,
      isBan: false,
      index: props.index,
    };
  } else {
    selection.value = null;
  }
}

function onDrop(event: DragEvent) {
  const championId = event.dataTransfer?.getData("championId");
  if (championId !== null) {
    const championIdNumber = Number(championId);
    emit("drop", championIdNumber);
  }
}

const emit = defineEmits(["dblclick", "drop"]);
</script>

<template>
  <button
    v-if="props.champion !== null"
    class="relative bg-cover bg-top"
    :style="`background-image: url(${props.champion.centered_default_skin_image_path})`"
    @click="updateSelection"
    @dblclick="$emit('dblclick')"
    @drop="onDrop($event)"
    @dragover.prevent
    @dragenter.prevent
  >
    <div
      class="absolute inset-0 border-zinc-100"
      :class="{
        border:
          !selection?.isBan &&
          selection?.index === index &&
          selection.team === props.team,
      }"
    ></div>
  </button>
  <button
    v-else
    class="relative bg-cover bg-top"
    @click="updateSelection"
    @drop="onDrop($event)"
    @dragover.prevent
    @dragenter.prevent
  >
    <div
      class="absolute inset-0 border"
      :class="{
        'border-zinc-100':
          !selection?.isBan &&
          selection?.index === index &&
          selection.team === props.team,
        'border-zinc-600':
          selection?.isBan ||
          selection?.index !== index ||
          selection.team !== props.team,
      }"
    ></div>
  </button>
</template>
