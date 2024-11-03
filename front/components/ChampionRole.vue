<script setup lang="ts">
interface Props {
  role: string;
}

const props = defineProps<Props>();

defineEmits(["click"]);

const roleSelected: Ref<string | null> = inject("roleSelected")!;
</script>

<template>
  <button
    :class="{
      'opacity-40': props.role !== roleSelected,
    }"
    @click="$emit('click')"
  >
    <slot />
  </button>
</template>
