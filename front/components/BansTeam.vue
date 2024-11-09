<script setup lang="ts">
import type { Champion } from "~/server/champion";
import type { ChampionDropData, Selection, Team } from "~/server/draft";

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

const emit = defineEmits<{
  dblclick: [index: number];
  drop: [championDropData: ChampionDropData];
}>();
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
  const origin = event.dataTransfer?.getData("origin");
  if (championId !== null) {
    const championIdNumber = Number(championId);
    const originParsed: Selection | null = origin ? JSON.parse(origin) : null;
    emit("drop", {
      newChampionId: championIdNumber,
      currentChampionId: props.champions[index]
        ? props.champions[index].id
        : null,
      newPosition: {
        team: props.team,
        index,
        isBan: true,
      },
      origin: originParsed,
    });
  }
}

function startDrag(event: DragEvent, championId: number, index: number) {
  if (event.dataTransfer !== null) {
    event.dataTransfer.dropEffect = "copy";
    event.dataTransfer.effectAllowed = "copy";
    event.dataTransfer.setData("championId", championId.toString());
    
    const origin: Selection = {
      team: props.team,
      index: index,
      isBan: true,
    };
    event.dataTransfer.setData("origin", JSON.stringify(origin));
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
        draggable
        @dragstart="startDrag($event, champion.id, index)"
        @click="updateSelection(index)"
        @dblclick="$emit('dblclick', index)"
        @drop="onDrop($event, index)"
        @dragover.prevent
        @dragenter.prevent
      >
        <img
          :src="champion.default_skin_image_path"
          class="absolute inset-0 h-full w-full object-cover object-top"
        />
        <div
          class="pointer-events-none absolute inset-0"
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
