<script setup lang="ts">
defineEmits(["click"]);
const { isSelected } = defineProps({
  isSelected: { type: Boolean, required: true },
});
</script>

<template>
  <button :class="{ 'bg-blue-100': isSelected }" @click="$emit('click')">
    <slot />
  </button>
</template>
