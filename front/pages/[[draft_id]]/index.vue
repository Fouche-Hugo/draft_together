<script setup lang="ts">
import { validate } from "uuid";

definePageMeta({
  validate: async (route) => {
    console.log(route.params.draft_id);
    return validate(route.params.draft_id);
  },
});
</script>

<template>
  <h1 class="bg-blue-100">{{ $route.params.draft_id }}</h1>
</template>
