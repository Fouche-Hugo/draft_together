<script setup lang="ts">
import { v4 } from "uuid";

function new_draft_link(): string {
  return "/" + v4();
}
</script>

<template>
  <NuxtLink :to="new_draft_link()">New draft</NuxtLink>
</template>
