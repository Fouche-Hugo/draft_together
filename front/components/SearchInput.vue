<script setup lang="ts">
const text = defineModel<string>();
</script>

<template>
  <div class="relative flex grow">
    <div
      class="pointer-events-none absolute inset-y-0 start-0 flex items-center ps-3"
    >
      <svg
        class="h-4 w-4 text-zinc-500 dark:text-zinc-400"
        aria-hidden="true"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 20 20"
      >
        <path
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
        />
      </svg>
    </div>
    <input
      v-model="text"
      class="grow border border-zinc-500 bg-transparent ps-10 text-lg text-zinc-300 focus:border-zinc-300 focus:outline-none"
      type="search"
    />
  </div>
</template>
