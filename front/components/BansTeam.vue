<script setup lang="ts">
import type { Champion } from "~/server/champion";
import type { Selection, Team } from "~/server/draft";

interface Props {
  champions: [
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
    Champion | null,
  ];
  team: Team;
}

const emit = defineEmits(["dblClick", "drop"]);
const props = defineProps<Props>();
const selection: Ref<Selection | null> = inject("selection")!;

function updateSelection(index: number) {
  if (
    selection.value?.team !== props.team ||
    !selection.value.isBan ||
    selection.value.index !== index
  ) {
    selection.value = {
      team: props.team,
      isBan: true,
      index,
    };
  } else {
    selection.value = null;
  }
}

function onDrop(event: DragEvent, index: number) {
  const championId = event.dataTransfer?.getData("championId");
  if (championId !== null) {
    const championIdNumber = Number(championId);
    emit("drop", championIdNumber, index);
  }
}
</script>

<template>
  <div class="grid h-full grid-cols-5">
    <template v-for="(champion, index) in props.champions" :key="index">
      <button
        v-if="champion !== null"
        class="relative h-24 w-24 bg-cover"
        :style="`background-image: url(${champion.default_skin_image_path})`"
        @click="updateSelection(index)"
        @dblclick="
          () => {
            console.log('dblClick');
            $emit('dblClick', index);
          }
        "
        @drop="onDrop($event, index)"
        @dragover.prevent
        @dragenter.prevent
      >
        <div
          class="absolute inset-0 border-zinc-100"
          :class="{
            border:
              selection?.isBan &&
              selection?.index === index &&
              selection.team === props.team,
          }"
        ></div>
      </button>
      <button
        v-else
        class="relative h-24 w-24 bg-cover"
        @click="updateSelection(index)"
        @drop="onDrop($event, index)"
        @dragover.prevent
        @dragenter.prevent
      >
        <div
          class="absolute inset-0 border"
          :class="{
            'border-zinc-100':
              selection?.isBan &&
              selection?.index === index &&
              selection.team === props.team,
            'border-zinc-600':
              !selection?.isBan ||
              selection?.index !== index ||
              selection.team !== props.team,
          }"
        ></div>
      </button>
    </template>
  </div>
</template>
