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
  const origin = event.dataTransfer?.getData("origin");
  if (championId !== null) {
    const championIdNumber = Number(championId);
    const originParsed: Selection | null = origin ? JSON.parse(origin) : null;

    emit(
      "drop",
      championIdNumber,
      props.champion ? props.champion.id : null,
      originParsed,
    );
  }
}

function startDrag(event: DragEvent) {
  if (event.dataTransfer !== null && props.champion !== null) {
    event.dataTransfer.dropEffect = "copy";
    event.dataTransfer.effectAllowed = "copy";
    event.dataTransfer.setData("championId", props.champion.id.toString());

    const origin: Selection = {
      team: props.team,
      index: props.index,
      isBan: false,
    };
    event.dataTransfer.setData("origin", JSON.stringify(origin));
  }
}

const emit = defineEmits<{
  dblclick: [];
  drop: [
    newChampionId: number,
    currentChampionId: number | null,
    origin: Selection | null,
  ];
}>();
</script>

<template>
  <div
    v-if="props.champion !== null"
    class="relative"
    draggable
    @dragstart="startDrag($event)"
    @click="updateSelection"
    @dblclick="$emit('dblclick')"
    @drop="onDrop($event)"
    @dragover.prevent
    @dragenter.prevent
  >
    <img
      :src="props.champion.centered_default_skin_image_path"
      class="absolute inset-0 h-full w-full object-cover object-top"
    />
    <div
      class="pointer-events-none absolute inset-0"
      :class="{
        border:
          !selection?.isBan &&
          selection?.index === index &&
          selection.team === props.team,
      }"
    ></div>
  </div>
  <div
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
  </div>
</template>
