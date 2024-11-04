<script setup lang="ts">
import { v4 } from "uuid";

function new_draft_link(): string {
  return "/" + v4();
}
</script>

<template>
  <main
    class="flex h-full flex-col items-center justify-between gap-8 text-zinc-100"
  >
    <div class="flex grow flex-col justify-center gap-8">
      <h1 class="text-3xl">Draft Together</h1>
      <section class="flex flex-col gap-4">
        <NuxtLink
          :to="new_draft_link()"
          class="mb-2 me-2 rounded-lg bg-zinc-700 p-1 px-5 py-2.5 text-center text-sm font-medium text-white hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-zinc-300"
          >Create a new draft</NuxtLink
        >
      </section>
    </div>
    <DraftFooter class="justify-self-end" />
  </main>
</template>
